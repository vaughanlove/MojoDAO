import { Connection } from '@solana/web3.js'

export const createConnection = (endpoint: string) => {
    const connection = new Connection(endpoint, 'confirmed')
    console.log(connection)
    return connection
}

export class GreetingAccount {
    counter = 0;
    constructor(fields: {counter: number} | undefined = undefined) {
        if (fields) {
            this.counter = fields.counter;
        }
    }
}

export const GreetingSchema = new Map([
    [GreetingAccount, {kind: 'struct', fields: [['counter', 'u32']]}],
]);

export default {createConnection, GreetingSchema, GreetingAccount}




