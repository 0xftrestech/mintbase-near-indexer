table! {
		use diesel::sql_types::*;
	stores (
		id,
		owner,
		name,
		symbol,
	) {
		id -> Text,
		owner -> Text,
		name -> Text,
		symbol -> Text,
		totalSupply -> BigInt,
		burned -> Bool,
		tokenCount -> Numeric,
		boughtCount -> Numeric,
		valueCount -> Numeric,
		transferCount -> Numeric,
		}
}

table! {
things (
	id,
	minter,
	burned,
	forSale,
	metaId,
	resolveStore
) {
	id -> Text,
	minter -> Text,
	burned -> Bool,
	forSale -> Bool,
	metaId -> Text,
	resolveStore -> Text,
	}
}

table! {
tokens (
	id,
	tokenId,
	metaId,
	price,
	burend,
	state,
	transferCount,
) {
	id -> Text,
	tokenId -> Text,
	metaId -> Text,
	price -> Text,
	burend -> Bool,
	state -> Text,
	transferCount -> BigInt,
	}
}

table! {
users (
	id,
	avitar,
) {
	id -> Text,
	avitar -> Text,
	}
}
