(function() {var implementors = {
"common_primitives":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"common_primitives/messages/struct.MessageResponse.html\" title=\"struct common_primitives::messages::MessageResponse\">MessageResponse</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"common_primitives/messages/struct.BlockPaginationRequest.html\" title=\"struct common_primitives::messages::BlockPaginationRequest\">BlockPaginationRequest</a>"],["impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"common_primitives/messages/struct.BlockPaginationResponse.html\" title=\"struct common_primitives::messages::BlockPaginationResponse\">BlockPaginationResponse</a>&lt;T&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"common_primitives/msa/struct.DelegatorId.html\" title=\"struct common_primitives::msa::DelegatorId\">DelegatorId</a>"],["impl&lt;SchemaId:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, BlockNumber:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, MaxSchemaGrantsPerDelegation:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"common_primitives/msa/struct.Delegation.html\" title=\"struct common_primitives::msa::Delegation\">Delegation</a>&lt;SchemaId, BlockNumber, MaxSchemaGrantsPerDelegation&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;MaxSchemaGrantsPerDelegation: Get&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>&gt;,</span>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"common_primitives/msa/struct.ProviderId.html\" title=\"struct common_primitives::msa::ProviderId\">ProviderId</a>"],["impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"common_primitives/msa/struct.ProviderRegistryEntry.html\" title=\"struct common_primitives::msa::ProviderRegistryEntry\">ProviderRegistryEntry</a>&lt;T&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Get&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>&gt;,</span>"],["impl&lt;AccountId:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"common_primitives/msa/struct.KeyInfoResponse.html\" title=\"struct common_primitives::msa::KeyInfoResponse\">KeyInfoResponse</a>&lt;AccountId&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"common_primitives/parquet/base/enum.ParquetBaseType.html\" title=\"enum common_primitives::parquet::base::ParquetBaseType\">ParquetBaseType</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"common_primitives/parquet/column/struct.ParquetColumn.html\" title=\"struct common_primitives::parquet::column::ParquetColumn\">ParquetColumn</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"common_primitives/parquet/column_compression_codec/enum.ColumnCompressionCodec.html\" title=\"enum common_primitives::parquet::column_compression_codec::ColumnCompressionCodec\">ColumnCompressionCodec</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"common_primitives/parquet/numeric/enum.ParquetNumericType.html\" title=\"enum common_primitives::parquet::numeric::ParquetNumericType\">ParquetNumericType</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"common_primitives/parquet/numeric/struct.ParquetInteger.html\" title=\"struct common_primitives::parquet::numeric::ParquetInteger\">ParquetInteger</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"common_primitives/parquet/numeric/struct.ParquetDecimal.html\" title=\"struct common_primitives::parquet::numeric::ParquetDecimal\">ParquetDecimal</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"common_primitives/parquet/string/enum.ParquetStringType.html\" title=\"enum common_primitives::parquet::string::ParquetStringType\">ParquetStringType</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"common_primitives/parquet/temporal/enum.ParquetTemporalType.html\" title=\"enum common_primitives::parquet::temporal::ParquetTemporalType\">ParquetTemporalType</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"common_primitives/parquet/temporal/struct.ParquetTime.html\" title=\"struct common_primitives::parquet::temporal::ParquetTime\">ParquetTime</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"common_primitives/parquet/temporal/struct.ParquetTimestamp.html\" title=\"struct common_primitives::parquet::temporal::ParquetTimestamp\">ParquetTimestamp</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"common_primitives/parquet/types/enum.ParquetType.html\" title=\"enum common_primitives::parquet::types::ParquetType\">ParquetType</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"common_primitives/schema/enum.ModelType.html\" title=\"enum common_primitives::schema::ModelType\">ModelType</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"common_primitives/schema/enum.PayloadLocation.html\" title=\"enum common_primitives::schema::PayloadLocation\">PayloadLocation</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"common_primitives/schema/struct.SchemaResponse.html\" title=\"struct common_primitives::schema::SchemaResponse\">SchemaResponse</a>"]],
"common_runtime":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"common_runtime/constants/struct.MaxDataSize.html\" title=\"struct common_runtime::constants::MaxDataSize\">MaxDataSize</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"common_runtime/constants/struct.MessagesMaxPayloadSizeBytes.html\" title=\"struct common_runtime::constants::MessagesMaxPayloadSizeBytes\">MessagesMaxPayloadSizeBytes</a>"],["impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + Config&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"common_runtime/extensions/check_nonce/struct.CheckNonce.html\" title=\"struct common_runtime::extensions::check_nonce::CheckNonce\">CheckNonce</a>&lt;T&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T::Index: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"]],
"frequency_cli":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"frequency_cli/struct.ExportMetadataCmd.html\" title=\"struct frequency_cli::ExportMetadataCmd\">ExportMetadataCmd</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"frequency_cli/struct.ExportRuntimeVersionCmd.html\" title=\"struct frequency_cli::ExportRuntimeVersionCmd\">ExportRuntimeVersionCmd</a>"]],
"frequency_rococo_runtime":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"frequency_rococo_runtime/struct.SessionKeys.html\" title=\"struct frequency_rococo_runtime::SessionKeys\">SessionKeys</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"frequency_rococo_runtime/struct.Runtime.html\" title=\"struct frequency_rococo_runtime::Runtime\">Runtime</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"frequency_rococo_runtime/enum.Event.html\" title=\"enum frequency_rococo_runtime::Event\">Event</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"frequency_rococo_runtime/struct.Origin.html\" title=\"struct frequency_rococo_runtime::Origin\">Origin</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"frequency_rococo_runtime/enum.OriginCaller.html\" title=\"enum frequency_rococo_runtime::OriginCaller\">OriginCaller</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"frequency_rococo_runtime/enum.Call.html\" title=\"enum frequency_rococo_runtime::Call\">Call</a>"]],
"frequency_runtime":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"frequency_runtime/struct.SessionKeys.html\" title=\"struct frequency_runtime::SessionKeys\">SessionKeys</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"frequency_runtime/struct.Runtime.html\" title=\"struct frequency_runtime::Runtime\">Runtime</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"frequency_runtime/enum.Event.html\" title=\"enum frequency_runtime::Event\">Event</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"frequency_runtime/struct.Origin.html\" title=\"struct frequency_runtime::Origin\">Origin</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"frequency_runtime/enum.OriginCaller.html\" title=\"enum frequency_runtime::OriginCaller\">OriginCaller</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"frequency_runtime/enum.Call.html\" title=\"enum frequency_runtime::Call\">Call</a>"]],
"frequency_service":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"frequency_service/chain_spec/struct.Extensions.html\" title=\"struct frequency_service::chain_spec::Extensions\">Extensions</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"frequency_service/chain_spec/struct.ExtensionsFork.html\" title=\"struct frequency_service::chain_spec::ExtensionsFork\">ExtensionsFork</a>"]],
"pallet_messages":[["impl&lt;MaxDataSize:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pallet_messages/struct.Message.html\" title=\"struct pallet_messages::Message\">Message</a>&lt;MaxDataSize&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;MaxDataSize: Get&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pallet_messages/pallet/struct.Pallet.html\" title=\"struct pallet_messages::pallet::Pallet\">Pallet</a>&lt;T&gt;"],["impl&lt;T:&nbsp;<a class=\"trait\" href=\"pallet_messages/pallet/trait.Config.html\" title=\"trait pallet_messages::pallet::Config\">Config</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"pallet_messages/pallet/enum.Event.html\" title=\"enum pallet_messages::pallet::Event\">Event</a>&lt;T&gt;"],["impl&lt;T:&nbsp;<a class=\"trait\" href=\"pallet_messages/pallet/trait.Config.html\" title=\"trait pallet_messages::pallet::Config\">Config</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"pallet_messages/pallet/enum.Call.html\" title=\"enum pallet_messages::pallet::Call\">Call</a>&lt;T&gt;"]],
"pallet_msa":[["impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"pallet_msa/pallet/trait.Config.html\" title=\"trait pallet_msa::pallet::Config\">Config</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pallet_msa/types/struct.AddKeyData.html\" title=\"struct pallet_msa::types::AddKeyData\">AddKeyData</a>&lt;T&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T::BlockNumber: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;T::AccountId: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pallet_msa/types/struct.AddProvider.html\" title=\"struct pallet_msa::types::AddProvider\">AddProvider</a>"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pallet_msa/pallet/struct.Pallet.html\" title=\"struct pallet_msa::pallet::Pallet\">Pallet</a>&lt;T&gt;"],["impl&lt;T:&nbsp;<a class=\"trait\" href=\"pallet_msa/pallet/trait.Config.html\" title=\"trait pallet_msa::pallet::Config\">Config</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"pallet_msa/pallet/enum.Event.html\" title=\"enum pallet_msa::pallet::Event\">Event</a>&lt;T&gt;"],["impl&lt;T:&nbsp;<a class=\"trait\" href=\"pallet_msa/pallet/trait.Config.html\" title=\"trait pallet_msa::pallet::Config\">Config</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"pallet_msa/pallet/enum.Call.html\" title=\"enum pallet_msa::pallet::Call\">Call</a>&lt;T&gt;"],["impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"pallet_msa/pallet/trait.Config.html\" title=\"trait pallet_msa::pallet::Config\">Config</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pallet_msa/struct.CheckFreeExtrinsicUse.html\" title=\"struct pallet_msa::CheckFreeExtrinsicUse\">CheckFreeExtrinsicUse</a>&lt;T&gt;"]],
"pallet_schemas":[["impl&lt;MaxModelSize:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pallet_schemas/struct.Schema.html\" title=\"struct pallet_schemas::Schema\">Schema</a>&lt;MaxModelSize&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;MaxModelSize: Get&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>&gt;,</span>"],["impl&lt;T:&nbsp;<a class=\"trait\" href=\"pallet_schemas/pallet/trait.Config.html\" title=\"trait pallet_schemas::pallet::Config\">Config</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"pallet_schemas/pallet/enum.Event.html\" title=\"enum pallet_schemas::pallet::Event\">Event</a>&lt;T&gt;"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pallet_schemas/pallet/struct.Pallet.html\" title=\"struct pallet_schemas::pallet::Pallet\">Pallet</a>&lt;T&gt;"],["impl&lt;T:&nbsp;<a class=\"trait\" href=\"pallet_schemas/pallet/trait.Config.html\" title=\"trait pallet_schemas::pallet::Config\">Config</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"pallet_schemas/pallet/enum.Call.html\" title=\"enum pallet_schemas::pallet::Call\">Call</a>&lt;T&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()