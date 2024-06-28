TODO!

SECTIONS:

PROGRAM:
    state / authority
        program authority
    implented:
        initialize
        add bid token mint

SESSION INSTANCE
    state:
        create vested config by session
    token account:
        commit bid vault -> must be accepted bid token mint
        create vested escrow by session
    implemented:
        create::
            session
            sealed_bid_round
            commit bit vault
            commit leader board
            reallocate commit leader board
            commit queue
            tick bid round
            vested token escrow
            vested config
            project token escrow
            token stake vault
    todo:
        create session leader board     -> zero copy linked list
            state account
        create round leader board 10x   -> zero copy linked list
            state account
    refactor
        change to anchor 0.30.0 -- done
        token interface -- done

SEALED-BID SYSTEM
    state
        stake vault state []
        sealed bid account [] -> note, this may contain the state for stake vault -> {INVESTOR}
        commit bid leader board -> {INSTANCE}
        commit bid queue -> {INSTANCE}
    token account
        stake vault
        commit bid vault
    implemented
        submit sealed bid
        submit unsealed bid
        submit commit bid
    todo
        refund commit bid -- done 
        unlock stake -- done
        update STATE::commit_bid_leader_board -> zero copy 
            - doesn't need to be zero copy, using the heap is enough to handle this account type

TICK-BID SYSTEM
    state
        round leader board -> {ROUND_INSTANCE}
        session leader board -> {INSTANCE}
        round status -> {INSTANCE}
        session status -> {INSTANCE}
        vesting status config -> {}
        vesting account -> {INVESTOR}
    token account
        vesting escrow
        project vault
    implemented
        register bid
    todo
        open bid
        execute bid -> algorithmic complexity.
        update leader board
    update / todo
        update STATE::tick_bid_session_leader_board -> zero copy 
        update STATE::tick_bid_round_leader_board -> zero copy
    non-mvp / strectch goal?
        match bid -> marketplace intergration -> non-MVP feature


VESTING SYSTEM 
    state
        vesting status config
        vesting / status account
    token account
        vesting escrow
        investor account
    implemented
    todo
        claim vested tokens
    non-mvp
        ... additional features to incentive reducing vesting period -> non-MVP feature

MARKETPLACE SYSTEM
    Todo: non-mvp


THOUGHTS:
    commit bid vault -> previous name -> create commit token account
        - 
        // THOUGHTS:
        // : if there is going to be more than one token that will be used to make bids, then there
        // will need to be multiple commit token accounts.
        // : the argument for using allowing to have multiple stable coins for user options to use for bids,
        // is availabitlity and liquidity. what if at a given point there is not
        // 1 million USDC at that moment available for users to place there bids.
        // I doubt such a situation would happen but there is a possibility.
        // having multiple stable coins as an option provides more liquidity and reduces that risk.
        // FINAL DECISION:
        // the commit token account will be ephemeral, instance based to a specific active session.
        // and it will be created only with USDC token mint. if in the future, there needs to be
        // used with optional token mints of other stable coins then that change can be made in the future.

        // this should be moved into session for sealed bid?
        // right now only one instance of this is being created
        // multiple instances should  be created for each session.
        // program authority is the authority of account but should that be true?
        // now that I think about it. this is for the commit queue.
        // so then maybe only one instances is okay to exist?
        // and is using a valid bid token mint -> USDC / SOL / Stable coin

        // should come back to this. right now there is only a single top level
        // instances of this account being created...
        // should there be only one instances or multiple instances at the session level?
        // is being created for the bid token... need add validation that this
        // can only be created with a valid bid token which is USDC ATM.