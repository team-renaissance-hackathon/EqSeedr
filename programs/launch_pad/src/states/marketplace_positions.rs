use crate::utils::*;
use anchor_lang::prelude::*;

#[account]
pub struct MarketplacePositions {
    pool: MarketplacePositionsLinkedList,
}

impl MarketplacePositions {
    pub const LEN: usize = DISCRIMINATOR + MarketplacePositionsLinkedList::LEN;

    pub fn initialize(&mut self) {
        self.pool = MarketplacePositionsLinkedList::new();
    }

    // need to set the values correctly from input
    // fn add(&self) {
    //     self.pool.add(
    //         pos,
    //         Node {
    //             index: 0,
    //             prev: None,
    //             next: None,
    //             position: MarketPosition {
    //                 index: 0,
    //                 vested_index: 0,
    //                 balance: 0,
    //                 bid_target: 0,
    //                 fee_pay: 0,
    //             },
    //         },
    //     )
    // }

    // validate 2 scenarios
    // if position changes, add the delta, check
    // if positions doesn't change, add the delta, then check
    // fn update(&mut self, params: Params) -> Result<()> {
    //     if params.current_pos != params.new_pos {
    //         self.pool.swap(params.current_pos, params.new_pos)
    //     }

    //     let node = &mut self.get(pos).unwrap();

    //     if bid_target_delta != 0 {
    //         node.position.bid_target = node.position.bid_target + params.bid_target_delta;
    //     }

    //     if fee_payout_delta != 0 {
    //         node.position.fee_payout = node.position.fee_payout + params.fee_payout_delta;
    //     }

    //     if balance_delta != 0 {
    //         node.position.balance += params.balance_delta;

    //         let cpi_program = params.token_program.to_account_info();

    //         let cpi_accounts = Transfer {
    //             from: params.matcher_token_account.to_account_info(),
    //             to: params.stake_token_account.to_account_info(),
    //             authority: params.authority.to_account_info(),
    //         };

    //         let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    //         transfer(cpi_ctx, params.balance_delta)?;
    //     }

    //     return self.pool.update(node);
    // }

    // fn remove(&self, pos: u32) {
    //     // self.pool.set_to_none(pos)
    // }

    // fn pos_is_valid(pos: u32) -> bool {
    //     return pos >= self.pool.total && self.pool.stack.is_empty()
    //         || (pos < self.pool.total && self.get(pos) == None && pos == self.pool.last_element());
    // }

    // fn node_is_valid(pos: u32, node: Node) {
    //     if pos == self.pool.head {
    //         let head_node = self.get(self.pool.head as usize).unwrap();

    //         return !(node.position.bid_target < head_node.position.bid_target);
    //     } else if (!self.pool.stack.is_empty() && self.get(pos as usize).is_none())
    //         || pos >= self.pool.total
    //     {
    //         let tail_node = self.get(self.tail as usize).unwrap();

    //         return !(node.position.bid_target >= tail_node.position.bid_target);
    //     } else {
    //         let next_node = self.get(pos as usize).unwrap();
    //         let prev_node = self.get(next_node.prev.unwrap() as usize).unwrap();

    //         return !(node.position.bid_target < next_node.position.bid_target)
    //             && !(node.position.bid_target >= prev_node.position.bid_target);
    //     }
    // }

    // // does this matter? I think I don't need it
    // fn valid_target_bid_update(
    //     &self,
    //     target_bid_delta: u64,
    //     current_pos: u32,
    //     new_pos: u32,
    // ) -> bool {
    //     return current_pos != new_pos
    //         && target_bid_delta != 0
    //         && self.pos_is_valid(new_pos, self.get(current_pos).unwrap());
    // }

    // fn get(&self, pos: u32) -> Option<Node> {
    //     return self.pool.list[pos].clone();
    // }
}

// LINKED LIST
#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct MarketplacePositionsLinkedList {
    total: u32,
    head: u32,
    tail: u32,
    list: Vec<MarketPositionNode>,
    stack: Vec<[u8; 3]>,
}

impl MarketplacePositionsLinkedList {
    pub const LEN: usize = UNSIGNED_32
        + UNSIGNED_32
        + UNSIGNED_64
        + (UNSIGNED_64 + MarketPositionNode::LEN)
        + (UNSIGNED_64 + (BYTE * 3));

    pub fn new() -> Self {
        MarketplacePositionsLinkedList {
            total: 0,
            head: 0,
            tail: 0,
            list: Vec::<MarketPositionNode>::new(),
            stack: Vec::<[u8; 3]>::new(),
        }
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct MarketPositionNode {
    index: u32,
    prev: Option<u32>,
    next: Option<u32>,
    position: MarketPosition,
}

impl MarketPositionNode {
    pub const LEN: usize =
        UNSIGNED_32 + (BYTE + UNSIGNED_32) + (BYTE + UNSIGNED_32) + MarketPosition::LEN;
}

// MARKET MAKER
#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct MarketPosition {
    index: u32,
    vested_index: u32,
    balance: u64,
    bid_target: u64,
    fee_pay: u64,
}

impl MarketPosition {
    pub const LEN: usize = UNSIGNED_32 + UNSIGNED_32 + UNSIGNED_64 + UNSIGNED_64 + UNSIGNED_64;
}
