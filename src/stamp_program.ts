import { PublicKey } from '@solana/web3.js';
import { Program } from '@metaplex-foundation/mpl-core';
import bs58 from 'bs58';
import { Stamp } from './accounts';

export class StampProgram extends Program {
  static readonly PUBKEY = new PublicKey('DkhsdcwKejLofq1VhehWdfua6gZweAvvog8RaMPxwbt');

  static findStampAccount(reference: string): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from(Stamp.PREFIX), bs58.decode(reference)],
      StampProgram.PUBKEY,
    );
  }
}
