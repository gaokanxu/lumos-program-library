#![cfg(feature = "test-sbf")]
use {
    lumos_program::{hash::hashv, instruction::Instruction, program_pack::Pack, pubkey::Pubkey},
    lumos_program_test::{
        processor, tokio, ProgramTest, ProgramTestBanksClientExt, ProgramTestContext,
    },
    lumos_sdk::{
        signature::{Keypair, Signer},
        transaction::Transaction,
        transport::TransportError,
    },
    lpl_name_service::{
        instruction::{create, delete, realloc, transfer, update, NameRegistryInstruction},
        processor::Processor,
        state::{get_seeds_and_key, NameRecordHeader, HASH_PREFIX},
    },
    std::str::FromStr,
};

#[tokio::test]
async fn test_name_service() {
    // Create program and test environment
    let program_id = Pubkey::from_str("XCWuBvfNamesXCWuBvfkegQfZyiNwAJb9Ss623VQ5DA").unwrap();

    let program_test = ProgramTest::new(
        "lpl_name_service",
        program_id,
        processor!(Processor::process_instruction),
    );

    let mut ctx = program_test.start_with_context().await;

    let root_name = ".sol";
    let tld_class = Keypair::new();
    let owner = Keypair::new();

    let hashed_root_name: Vec<u8> = hashv(&[(HASH_PREFIX.to_owned() + root_name).as_bytes()])
        .as_ref()
        .to_vec();
    let (root_name_account_key, _) = get_seeds_and_key(
        &program_id,
        hashed_root_name.clone(),
        Some(&tld_class.pubkey()),
        None,
    );

    let space = 1_000usize;
    let rent = ctx.banks_client.get_rent().await.unwrap();
    let create_name_instruction = create(
        program_id,
        NameRegistryInstruction::Create {
            hashed_name: hashed_root_name,
            lamports: rent.minimum_balance(space.saturating_add(NameRecordHeader::LEN)),
            space: space as u32,
        },
        root_name_account_key,
        ctx.payer.pubkey(),
        owner.pubkey(),
        Some(tld_class.pubkey()),
        None,
        None,
    )
    .unwrap();
    sign_send_instruction(&mut ctx, create_name_instruction, vec![&tld_class])
        .await
        .unwrap();

    let name_record_header = NameRecordHeader::unpack_from_slice(
        &ctx.banks_client
            .get_account(root_name_account_key)
            .await
            .unwrap()
            .unwrap()
            .data,
    )
    .unwrap();
    println!("Name Record Header: {:?}", name_record_header);

    let name = "bonfida";
    let sol_subdomains_class = Keypair::new();

    let hashed_name: Vec<u8> = hashv(&[(HASH_PREFIX.to_owned() + name).as_bytes()])
        .as_ref()
        .to_vec();
    let (name_account_key, _) = get_seeds_and_key(
        &program_id,
        hashed_name.clone(),
        Some(&sol_subdomains_class.pubkey()),
        Some(&root_name_account_key),
    );

    let create_name_instruction = create(
        program_id,
        NameRegistryInstruction::Create {
            hashed_name,
            lamports: rent.minimum_balance(space.saturating_add(NameRecordHeader::LEN)),
            space: space as u32,
        },
        name_account_key,
        ctx.payer.pubkey(),
        owner.pubkey(),
        Some(sol_subdomains_class.pubkey()),
        Some(root_name_account_key),
        Some(owner.pubkey()),
    )
    .unwrap();
    sign_send_instruction(
        &mut ctx,
        create_name_instruction,
        vec![&sol_subdomains_class, &owner],
    )
    .await
    .unwrap();

    let name_record_header = NameRecordHeader::unpack_from_slice(
        &ctx.banks_client
            .get_account(name_account_key)
            .await
            .unwrap()
            .unwrap()
            .data,
    )
    .unwrap();
    println!("Name Record Header: {:?}", name_record_header);
    println!("SOl class {:?}", sol_subdomains_class.pubkey());

    let data = "@Dudl".as_bytes().to_vec();
    let update_instruction = update(
        program_id,
        0,
        data,
        name_account_key,
        sol_subdomains_class.pubkey(),
        Some(name_record_header.parent_name),
    )
    .unwrap();
    sign_send_instruction(&mut ctx, update_instruction, vec![&sol_subdomains_class])
        .await
        .unwrap();

    let name_record_header = NameRecordHeader::unpack_from_slice(
        &ctx.banks_client
            .get_account(name_account_key)
            .await
            .unwrap()
            .unwrap()
            .data,
    )
    .unwrap();
    println!("Name Record Header: {:?}", name_record_header);

    let transfer_instruction = transfer(
        program_id,
        ctx.payer.pubkey(),
        name_account_key,
        owner.pubkey(),
        Some(sol_subdomains_class.pubkey()),
    )
    .unwrap();
    sign_send_instruction(
        &mut ctx,
        transfer_instruction,
        vec![&owner, &sol_subdomains_class],
    )
    .await
    .unwrap();

    let name_record_header = NameRecordHeader::unpack_from_slice(
        &ctx.banks_client
            .get_account(name_account_key)
            .await
            .unwrap()
            .unwrap()
            .data,
    )
    .unwrap();
    println!("Name Record Header: {:?}", name_record_header);

    let data = "hello".as_bytes().to_vec();
    let update_instruction = update(
        program_id,
        space as u32,
        data,
        name_account_key,
        sol_subdomains_class.pubkey(),
        Some(name_record_header.parent_name),
    )
    .unwrap();

    sign_send_instruction(
        &mut ctx,
        update_instruction.clone(),
        vec![&sol_subdomains_class],
    )
    .await
    .unwrap_err();

    let new_space = space.checked_mul(2).unwrap();
    let payer_key = ctx.payer.pubkey();
    let realloc_instruction = |space| {
        realloc(
            program_id,
            payer_key,
            name_account_key,
            payer_key,
            space as u32,
        )
        .unwrap()
    };

    sign_send_instruction(&mut ctx, realloc_instruction(new_space), vec![])
        .await
        .unwrap();

    // update blockhash to prevent losing txn to dedup
    ctx.last_blockhash = ctx
        .banks_client
        .get_new_latest_blockhash(&ctx.last_blockhash)
        .await
        .unwrap();

    // resend update ix. Should succeed this time.
    sign_send_instruction(&mut ctx, update_instruction, vec![&sol_subdomains_class])
        .await
        .unwrap();

    // realloc to smaller this time
    sign_send_instruction(&mut ctx, realloc_instruction(space / 2), vec![])
        .await
        .unwrap();

    let delete_instruction = delete(
        program_id,
        name_account_key,
        ctx.payer.pubkey(),
        ctx.payer.pubkey(),
    )
    .unwrap();
    sign_send_instruction(&mut ctx, delete_instruction, vec![])
        .await
        .unwrap();
}

// Utils
pub async fn sign_send_instruction(
    ctx: &mut ProgramTestContext,
    instruction: Instruction,
    signers: Vec<&Keypair>,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&ctx.payer.pubkey()));
    let mut payer_signers = vec![&ctx.payer];
    for s in signers {
        payer_signers.push(s);
    }
    transaction.partial_sign(&payer_signers, ctx.last_blockhash);
    ctx.banks_client
        .process_transaction(transaction)
        .await
        .map_err(|e| e.into())
}
