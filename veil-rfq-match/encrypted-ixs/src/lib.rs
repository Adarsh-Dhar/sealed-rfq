use arcis::*;

#[encrypted]
mod circuits {
    use arcis::*;

    pub struct SealedQuote {
        maker_index: u8,
        price_bps: u32,
    }

    pub struct BestQuote {
        winner_index: u8,
        winner_price_bps: u32,
    }

    #[instruction]
    pub fn select_best_quote(
        quote_a: Enc<Shared, SealedQuote>,
        quote_b: Enc<Shared, SealedQuote>,
    ) -> Enc<Shared, BestQuote> {
        let a = quote_a.to_arcis();
        let b = quote_b.to_arcis();

        let a_wins = a.price_bps < b.price_bps; // lowest price wins, matches lib/rfq.ts
        let winner_index = a_wins.select(a.maker_index, b.maker_index);
        let winner_price_bps = a_wins.select(a.price_bps, b.price_bps);

        quote_a.owner.from_arcis(BestQuote {
            winner_index,
            winner_price_bps,
        })
    }
}
