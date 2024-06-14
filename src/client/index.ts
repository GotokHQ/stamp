import {
  PublicKey,
  TransactionInstruction,
  SYSVAR_RENT_PUBKEY,
  SystemProgram,
} from '@solana/web3.js';
import { initializeStampInput } from './types';
import { StampProgram } from '../stamp_program';
import { InitStampArgs } from '../transactions';

export class StampClient {
  initStampInstruction = async (input: initializeStampInput) => {
    const authority = new PublicKey(input.authority);
    const feePayer = new PublicKey(input.feePayer);
    const reference = new PublicKey(input.reference);
    const [stamp, bump] = await StampProgram.findStampAccount(reference);
    const data = InitStampArgs.serialize({
      bump,
    });
    const keys = [
      {
        pubkey: authority,
        isSigner: true,
        isWritable: false,
      },
      {
        pubkey: feePayer,
        isSigner: true,
        isWritable: true,
      },
      {
        pubkey: stamp,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: reference,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: SYSVAR_RENT_PUBKEY,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
    ];
    return new TransactionInstruction({
      keys,
      data,
      programId: StampProgram.PUBKEY,
    });
  };
}
