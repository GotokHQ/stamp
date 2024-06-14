import { PublicKey } from '@solana/web3.js';
import { Program } from '@metaplex-foundation/mpl-core';
import { Stamp } from './accounts';

export class StampProgram extends Program {
  static readonly PUBKEY = new PublicKey('G8azYVro1VexvpWMuQzcQyPJxVgAWYp6pyAWQGJvVKHM');

  static async findStampAccount(reference: PublicKey): Promise<[PublicKey, number]> {
    return PublicKey.findProgramAddress(
      [Buffer.from(Stamp.PREFIX), reference.toBuffer()],
      StampProgram.PUBKEY,
    );
  }
}
