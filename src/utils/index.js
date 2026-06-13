export const debounce = (fn, delay = 500) => {
    let timer = null;
    const debounced = (...args) => {
        timer && clearTimeout(timer);
        timer = setTimeout(() => fn(...args), delay);
    };
    debounced.cancel = () => {
        timer && clearTimeout(timer);
        timer = null;
    };
    return debounced;
};
