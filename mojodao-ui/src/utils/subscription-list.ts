// https://github.com/MikeMcl/bignumber.js
// https://blog.csdn.net/shenxianhui1995/article/details/103985434

import {AccountInfo, PublicKey} from "@solana/web3.js";

export interface ProgramInfo {
    symbol: string
    name: string
    id: number

    programKey: PublicKey
    cost: number
    duration: number

    active: boolean

    subscribedUntil?: number

    referrer?: string

    details?: string
    website?: string

}

export const PROGRAMS: ProgramInfo[] = [
    {
        symbol: '../assets/Spotify-logo.png',
        name: 'Spotify - One month',
        id: 0,
        programKey: new PublicKey('EDTtxSugqhzghWB8aJtHoqktTfnBuTtbHFWYDGs6A8Rs'),
        cost: 12,
        duration: 30,
        active: false,

        details: 'the music subscription service spotify',
    },
    {
        symbol: '../assets/Spotify-logo.png',
        name: 'Spotify - Three month',
        id: 1,
        programKey: new PublicKey('6QZ5ehKAS4inTNJLxnLSU3AKUZ4MokMtTUYKjZmfvP1D'),
        cost: 30,
        duration: 90,
        active: false,

        details: 'the music subscription service spotify',
    },
    {
        symbol: '../assets/Spotify-logo.png',
        name: 'Spotify - Six month',
        id: 2,
        programKey: new PublicKey('J2sw9oeXQQVph9Di1wgUqu8PTdMn51y9FuYMgDEMMqNK'),
        cost: 30,
        duration: 90,
        active: false,

        details: 'the music subscription service spotify',
    },
    {
        symbol: '../assets/Spotify-logo.png',
        name: 'Spotify - Twelve month',
        id: 3,
        programKey: new PublicKey('6mREPwVPdQfUnmXwvfHQduPCmJVoDbXJe4s4eArTT2sW'),
        cost: 30,
        duration: 90,
        active: false,

        details: 'the music subscription service spotify',
    },
]
