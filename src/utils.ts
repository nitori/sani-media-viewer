export const posWithin = (itemEl: HTMLElement, parentEl: HTMLElement) => {
    let rect = itemEl.getBoundingClientRect();
    let parentRect = parentEl.getBoundingClientRect();
    return rect.top >= parentRect.top && rect.bottom <= parentRect.bottom;
}

export const focusElementInParent = (itemEl: HTMLElement, parentEl: HTMLElement) => {
    if (itemEl === null || parentEl === null) {
        return;
    }
    if (posWithin(itemEl, parentEl)) {
        return;
    }
    itemEl.scrollIntoView({behavior: 'instant', block: 'center'});
}