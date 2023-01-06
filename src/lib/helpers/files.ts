export const file_name = (path: string) => {
    return path.includes('\\') ? path.split('\\').pop() : path.split('/').pop()
}

export const file_ext = (path: string) => {
    return path.split('.').pop()
}