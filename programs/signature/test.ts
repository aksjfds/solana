import { createKeyPairFromBytes, getUtf8Encoder, signBytes } from '@solana/kit';
import { Keypair } from '@solana/web3.js';

(async () => {

    const bob = Keypair.generate();

    const bobsKeyPair = await createKeyPairFromBytes(
        bob.secretKey
    );
    


    const message = getUtf8Encoder().encode('The meeting is at 6:00pm');
    const signature = await signBytes(bobsKeyPair.privateKey, message);
    console.log(signature);
    
})()