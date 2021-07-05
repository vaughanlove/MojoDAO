export default function importSymbol(path: string) {
    try {
        return require(`../assets${path}`)
    } catch (e) {
        return require('../assets/icons/unknown.png')
    }
}