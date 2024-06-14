import {
  PublicKey,
  TransactionInstruction,
  SYSVAR_RENT_PUBKEY,
  SystemProgram,
} from '@solana/web3.js';
import { CreateStampInstructionParams } from './types';
import { StampProgram } from '../stamp_program';
import { InitStampArgs } from '../transactions';

export const createStampInstruction = (input: CreateStampInstructionParams) => {
  const feePayer = new PublicKey(input.feePayer);
  const [stamp, bump] = StampProgram.findStampAccount(input.reference);
  const data = InitStampArgs.serialize({
    bump,
    reference: input.reference,
  });
  const keys = [
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
