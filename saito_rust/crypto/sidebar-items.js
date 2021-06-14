initSidebarItems({"fn":[["hash_bytes","Hash the message byte Vec with sha256 for signing by secp256k1 and return as Sha256Hash"],["make_message_from_string","Hash the message string with sha256 for signing by secp256k1 and return as byte array"],["verify_bytes_message","Verify a message signed by secp256k1. Message is a byte array. Sig and pubkey should be base58 encoded."],["verify_string_message","Verify a message signed by secp256k1. Message is a plain string. Sig and pubkey should be base58 encoded."]],"static":[["SECP256K1","A global, static context to avoid repeatedly creating contexts where one can’t be passed"]],"struct":[["Message","A (hashed) message input to an ECDSA signature"],["PublicKey","A Secp256k1 public key, used for verification of signatures"],["Signature","An ECDSA signature"]],"type":[["Sha256Hash","Sha256Hash byte array type"]]});