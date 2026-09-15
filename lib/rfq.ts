export type Quote = { maker: string; priceBps: number; commitment: string };

export function selectBestEligibleQuote(quotes: Quote[], eligibleMakers: Set<string>): Quote {
  const eligible = quotes.filter(quote => eligibleMakers.has(quote.maker));
  if (eligible.length === 0) throw new Error("No eligible quotes");
  return eligible.reduce((best, quote) => quote.priceBps < best.priceBps ? quote : best);
}

export function rfqStatus(now: number, expiration: number, selected: boolean, settled: boolean): "Open" | "Expired" | "Selected" | "Settled" {
  if (settled) return "Settled";
  if (selected) return "Selected";
  return now > expiration ? "Expired" : "Open";
}
