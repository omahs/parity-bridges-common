// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]
// RuntimeApi generated functions
#![allow(clippy::too_many_arguments)]

use bp_messages::{
	InboundMessageDetails, LaneId, MessageNonce, MessagePayload, OutboundMessageDetails,
};
use bp_runtime::Chain;
use frame_support::{
	weights::{constants::WEIGHT_PER_SECOND, DispatchClass, IdentityFee, Weight},
	Parameter, RuntimeDebug,
};
use frame_system::limits;
use sp_core::Hasher as HasherT;
use sp_runtime::{
	traits::{BlakeTwo256, IdentifyAccount, Verify},
	FixedU128, MultiSignature, MultiSigner, Perbill,
};
use sp_std::vec::Vec;

/// Identifier of RialtoParachain in the Rialto relay chain.
///
/// This identifier is not something that is declared either by Rialto or RialtoParachain. This
/// is an identifier of registration. So in theory it may be changed. But since bridge is going
/// to be deployed after parachain registration AND since parachain de-registration is highly
/// likely impossible, it is fine to declare this constant here.
pub const RIALTO_PARACHAIN_ID: u32 = 2000;

/// Number of extra bytes (excluding size of storage value itself) of storage proof, built at
/// RialtoParachain chain. This mostly depends on number of entries (and their density) in the
/// storage trie. Some reserve is reserved to account future chain growth.
pub const EXTRA_STORAGE_PROOF_SIZE: u32 = 1024;

/// Can be computed by subtracting encoded call size from raw transaction size.
pub const TX_EXTRA_BYTES: u32 = 104;

/// Maximal weight of single RialtoParachain block.
///
/// This represents two seconds of compute assuming a target block time of six seconds.
pub const MAXIMUM_BLOCK_WEIGHT: Weight = 2 * WEIGHT_PER_SECOND;

/// Represents the average portion of a block's weight that will be used by an
/// `on_initialize()` runtime call.
pub const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(10);

/// Represents the portion of a block that will be used by Normal extrinsics.
pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

/// Maximal number of unrewarded relayer entries in Rialto confirmation transaction.
pub const MAX_UNREWARDED_RELAYERS_IN_CONFIRMATION_TX: MessageNonce = 1024;

/// Maximal number of unconfirmed messages in Rialto confirmation transaction.
pub const MAX_UNCONFIRMED_MESSAGES_IN_CONFIRMATION_TX: MessageNonce = 1024;

/// Weight of single regular message delivery transaction on RialtoParachain chain.
///
/// This value is a result of `pallet_bridge_messages::Pallet::receive_messages_proof_weight()` call
/// for the case when single message of `pallet_bridge_messages::EXPECTED_DEFAULT_MESSAGE_LENGTH`
/// bytes is delivered. The message must have dispatch weight set to zero. The result then must be
/// rounded up to account possible future runtime upgrades.
pub const DEFAULT_MESSAGE_DELIVERY_TX_WEIGHT: Weight = 1_500_000_000;

/// Increase of delivery transaction weight on RialtoParachain chain with every additional message
/// byte.
///
/// This value is a result of
/// `pallet_bridge_messages::WeightInfoExt::storage_proof_size_overhead(1)` call. The result then
/// must be rounded up to account possible future runtime upgrades.
pub const ADDITIONAL_MESSAGE_BYTE_DELIVERY_WEIGHT: Weight = 25_000;

/// Maximal weight of single message delivery confirmation transaction on RialtoParachain chain.
///
/// This value is a result of `pallet_bridge_messages::Pallet::receive_messages_delivery_proof`
/// weight formula computation for the case when single message is confirmed. The result then must
/// be rounded up to account possible future runtime upgrades.
pub const MAX_SINGLE_MESSAGE_DELIVERY_CONFIRMATION_TX_WEIGHT: Weight = 2_000_000_000;

/// Weight of pay-dispatch-fee operation for inbound messages at Rialto chain.
///
/// This value corresponds to the result of
/// `pallet_bridge_messages::WeightInfoExt::pay_inbound_dispatch_fee_overhead()` call for your
/// chain. Don't put too much reserve there, because it is used to **decrease**
/// `DEFAULT_MESSAGE_DELIVERY_TX_WEIGHT` cost. So putting large reserve would make delivery
/// transactions cheaper.
pub const PAY_INBOUND_DISPATCH_FEE_WEIGHT: Weight = 600_000_000;

/// Block number type used in Rialto.
pub type BlockNumber = u32;

/// Hash type used in Rialto.
pub type Hash = <BlakeTwo256 as HasherT>::Out;

/// The type of object that can produce hashes on Rialto.
pub type Hasher = BlakeTwo256;

/// The header type used by Rialto.
pub type Header = sp_runtime::generic::Header<BlockNumber, Hasher>;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Public key of the chain account that may be used to verify signatures.
pub type AccountSigner = MultiSigner;

/// Balance of an account.
pub type Balance = u128;

/// An instant or duration in time.
pub type Moment = u64;

/// Index of a transaction in the parachain.
pub type Index = u32;

/// Weight-to-Fee type used by Rialto parachain.
pub type WeightToFee = IdentityFee<Balance>;

/// Rialto parachain.
#[derive(RuntimeDebug)]
pub struct RialtoParachain;

impl Chain for RialtoParachain {
	type BlockNumber = BlockNumber;
	type Hash = Hash;
	type Hasher = Hasher;
	type Header = Header;

	type AccountId = AccountId;
	type Balance = Balance;
	type Index = Index;
	type Signature = Signature;

	fn max_extrinsic_size() -> u32 {
		*BlockLength::get().max.get(DispatchClass::Normal)
	}

	fn max_extrinsic_weight() -> Weight {
		BlockWeights::get()
			.get(DispatchClass::Normal)
			.max_extrinsic
			.unwrap_or(Weight::MAX)
	}
}

frame_support::parameter_types! {
	pub BlockLength: limits::BlockLength =
		limits::BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub BlockWeights: limits::BlockWeights = limits::BlockWeights::builder()
		// Allowance for Normal class
		.for_class(DispatchClass::Normal, |weights| {
			weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
		})
		// Allowance for Operational class
		.for_class(DispatchClass::Operational, |weights| {
			weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
			// Extra reserved space for Operational class
			weights.reserved = Some(MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
		})
		// By default Mandatory class is not limited at all.
		// This parameter is used to derive maximal size of a single extrinsic.
		.avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
		.build_or_panic();
}

/// Name of the With-Rialto-Parachain messages pallet instance that is deployed at bridged chains.
pub const WITH_RIALTO_PARACHAIN_MESSAGES_PALLET_NAME: &str = "BridgeRialtoParachainMessages";
/// Name of the transaction payment pallet at the Rialto parachain runtime.
pub const TRANSACTION_PAYMENT_PALLET_NAME: &str = "TransactionPayment";

/// Name of the Millau->RialtoParachain (actually KSM->DOT) conversion rate stored in the Rialto
/// parachain runtime.
pub const MILLAU_TO_RIALTO_PARACHAIN_CONVERSION_RATE_PARAMETER_NAME: &str =
	"MillauToRialtoParachainConversionRate";
/// Name of the Millau fee multiplier parameter, stored in the Rialto parachain runtime.
pub const MILLAU_FEE_MULTIPLIER_PARAMETER_NAME: &str = "MillauFeeMultiplier";

/// Name of the `RialtoParachainFinalityApi::best_finalized` runtime method.
pub const BEST_FINALIZED_RIALTO_PARACHAIN_HEADER_METHOD: &str =
	"RialtoParachainFinalityApi_best_finalized";

/// Name of the `ToRialtoParachainOutboundLaneApi::estimate_message_delivery_and_dispatch_fee`
/// runtime method.
pub const TO_RIALTO_PARACHAIN_ESTIMATE_MESSAGE_FEE_METHOD: &str =
	"ToRialtoParachainOutboundLaneApi_estimate_message_delivery_and_dispatch_fee";
/// Name of the `ToRialtoParachainOutboundLaneApi::message_details` runtime method.
pub const TO_RIALTO_PARACHAIN_MESSAGE_DETAILS_METHOD: &str =
	"ToRialtoParachainOutboundLaneApi_message_details";

/// Name of the `FromRialtoParachainInboundLaneApi::message_details` runtime method.
pub const FROM_RIALTO_PARACHAIN_MESSAGE_DETAILS_METHOD: &str =
	"FromRialtoParachainInboundLaneApi_message_details";

sp_api::decl_runtime_apis! {
	/// API for querying information about the finalized RialtoParachain headers.
	///
	/// This API is implemented by runtimes that are bridging with the RialtoParachain chain, not the
	/// RialtoParachain runtime itself.
	pub trait RialtoParachainFinalityApi {
		/// Returns number and hash of the best finalized header known to the bridge module.
		fn best_finalized() -> Option<bp_runtime::HeaderId<Hash, BlockNumber>>;
	}

	/// Outbound message lane API for messages that are sent to RialtoParachain chain.
	///
	/// This API is implemented by runtimes that are sending messages to RialtoParachain chain, not the
	/// RialtoParachain runtime itself.
	pub trait ToRialtoParachainOutboundLaneApi<OutboundMessageFee: Parameter, OutboundPayload: Parameter> {
		/// Estimate message delivery and dispatch fee that needs to be paid by the sender on
		/// this chain.
		///
		/// Returns `None` if message is too expensive to be sent to RialtoParachain from this chain.
		///
		/// Please keep in mind that this method returns the lowest message fee required for message
		/// to be accepted to the lane. It may be good idea to pay a bit over this price to account
		/// future exchange rate changes and guarantee that relayer would deliver your message
		/// to the target chain.
		fn estimate_message_delivery_and_dispatch_fee(
			lane_id: LaneId,
			payload: OutboundPayload,
			rialto_parachain_to_this_conversion_rate: Option<FixedU128>,
		) -> Option<OutboundMessageFee>;
		/// Returns dispatch weight, encoded payload size and delivery+dispatch fee of all
		/// messages in given inclusive range.
		///
		/// If some (or all) messages are missing from the storage, they'll also will
		/// be missing from the resulting vector. The vector is ordered by the nonce.
		fn message_details(
			lane: LaneId,
			begin: MessageNonce,
			end: MessageNonce,
		) -> Vec<OutboundMessageDetails<OutboundMessageFee>>;
	}

	/// Inbound message lane API for messages sent by RialtoParachain chain.
	///
	/// This API is implemented by runtimes that are receiving messages from RialtoParachain chain, not the
	/// RialtoParachain runtime itself.
	///
	/// Entries of the resulting vector are matching entries of the `messages` vector. Entries of the
	/// `messages` vector may (and need to) be read using `To<ThisChain>OutboundLaneApi::message_details`.
	pub trait FromRialtoParachainInboundLaneApi<InboundMessageFee: Parameter> {
		/// Return details of given inbound messages.
		fn message_details(
			lane: LaneId,
			messages: Vec<(MessagePayload, OutboundMessageDetails<InboundMessageFee>)>,
		) -> Vec<InboundMessageDetails>;
	}
}
