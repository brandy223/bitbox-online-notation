export function capitalizeFirstLetter(str: string): string {
    return str.charAt(0).toUpperCase() + str.slice(1);
}

export const showModal = (element_name: string) => {
    // @ts-ignore
    document.getElementById(element_name).showModal();
}

export const hideModal = (element_name: string) => {
    // @ts-ignore
    document.getElementById(element_name).close();
}