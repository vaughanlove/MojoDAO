import { reactive, toRefs } from '@vue/reactivity';
import SolanaWalletAdapter from "@project-serum/sol-wallet-adapter";

/**
 * Define our state
 *
 * This state is mutable in the function below,
 * but if we pass it to the component without `toRefs`
 * it will not mutate.
 */
const state = reactive({
    wall: new SolanaWalletAdapter('https://www.sollet.io', "https://api.devnet.solana.com"),
});

export default () => {

    /**
     * Return the state as a `reference` to make it mutable.
     */
    return {
        state: toRefs(state),
    };
};