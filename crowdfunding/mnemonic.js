const StellarHDWallet = require('stellar-hd-wallet');
const mnemonic = "scheme talk reflect junk juice laundry ten kingdom legal denial slight twist";
const wallet = StellarHDWallet.fromMnemonic(mnemonic);
console.log(wallet.getSecret(0)); // -> gives you your Sxxxxxxxx secret key
console.log(wallet.getPublicKey(0));