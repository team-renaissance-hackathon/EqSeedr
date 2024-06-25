TODO!

SECTIONS:
SESSION INSTANCE
    implemented:
        commit leader board -> box account on heap, no changes to linked list
    todo:
        create commit bid vault -> is rename of create commit token account
            token account   -> USDC
        create session leader board     -> zero copy linked list
            state account
        create round leader board 10x   -> zero copy linked list
            state account
        create vested config by session
            state
        create vested escrow by session
            token account   -> project token
    refactor
        change to anchor 0.30.0 -- done
        token interface -- done

SEALED-BID SYSTEM
    state
        stake vault state []
        sealed bid account [] -> note, this make contain the state for stake vault -> {INVESTOR}
        commit bid leader board -> {INSTANCE}
        commit bid queue -> {INSTANCE}
    token account
        stake vault
        commit bid escrow / vault
    implemented
        submit sealed bid
        submit unsealed bid
        submit commit bid
    todo
        refund commit bid -- done (partially?)
        unlock stake
        update STATE::commit_bid_leader_board -> zero copy 

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
    todo
        register bid
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