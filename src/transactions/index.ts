import { Borsh } from '@metaplex-foundation/mpl-core';

type Args = {
  bump: number;
  reference: string;
};

export class InitStampArgs extends Borsh.Data<Args> {
  static readonly SCHEMA = InitStampArgs.struct([
    ['instruction', 'u8'],
    ['bump', 'u8'],
    ['reference', 'string'],
  ]);

  instruction = 0;
  bump: number;
  reference: string;
}
