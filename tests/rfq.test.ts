import test from "node:test";
import assert from "node:assert/strict";
import { rfqStatus, selectBestEligibleQuote } from "../lib/rfq";

test("selects the lowest price among eligible makers", () => {
  const result = selectBestEligibleQuote([
    { maker: "northstar", priceBps: 9862, commitment: "a" },
    { maker: "horizon", priceBps: 9848, commitment: "b" },
    { maker: "unapproved", priceBps: 9700, commitment: "c" }
  ], new Set(["northstar", "horizon"]));
  assert.equal(result.maker, "horizon");
});

test("does not permit selection after expiry", () => {
  assert.equal(rfqStatus(101, 100, false, false), "Expired");
  assert.equal(rfqStatus(101, 100, true, false), "Selected");
});
