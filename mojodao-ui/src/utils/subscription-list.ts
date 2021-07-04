// https://github.com/MikeMcl/bignumber.js
// https://blog.csdn.net/shenxianhui1995/article/details/103985434

export interface ProgramInfo {
    symbol: string
    name: string

    programAddress: string
    cost: number
    duration: number

    referrer?: string

    details?: string
    website?: string

}

/**
 * Get token use symbol
 * @param {string} symbol
 * @returns {ProgramInfo | null} tokenInfo
 */


/**
 * Get token use mint addresses
 * @param {string} pAddress
 * @returns {ProgramInfo | null} tokenInfo
 */
export function getTokenByMintAddress(pAddress: string): ProgramInfo | null {
    const address = Object.values(PROGRAMS).find((item) => item.programAddress === pAddress)
    return address ? (address) : null
}

export interface Programs {
    [key: string]: any
    [index: number]: any
}

export const PROGRAMS: Programs = {
    SPOTIFY: {
        symbol: '/assets/assets/Spotify-logo.png',
        name: 'Spotify',
        programAddress: 'EDTtxSugqhzghWB8aJtHoqktTfnBuTtbHFWYDGs6A8Rs',
        cost: 12,
        duration: 30,

        details: 'the music subscription service spotify',
    },
}
