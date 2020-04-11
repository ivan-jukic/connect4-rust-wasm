/**
 * Maybe (aka Option) - simple implementation
 * One of the more ridiculous things in JavaScript, not only that you have to
 * deal with null values, but also undefined values
*/ 

export type Maybe<T> = T | undefined | null;


/**
 * Check that a value is not nothing. Not exported.
 * 
 * @param maybe - value that we'd like to check
 * @returns true | false
 */ 
function _check<T>(maybe: Maybe<T>): Boolean {
    return undefined != maybe && null != maybe;
}


/**
 * Initialise Maybe with value, throws error if value is "nothing".
 * 
 * @param value - value to init as Just
 * @returns well, the value... but what matters here is the check and type!
 */
export function Just<T>(value: T): Maybe<T> {
    if(!_check(value)) {
        throw Error("Provided value cannot be Just!");
    }

    return value;
}


/**
 * Initialise maybe with "nothing".
 * 
 * @returns null, which is interpreted as Nothing.
 */
export function Nothing<T>(): Maybe<T> {
    return null;
}


/**
 * Get value from Maybe, if no value is present return default.
 * 
 * @param default_  - value that will be used if second arg is Nothing
 * @param maybe     - value that will be unpacked into value T
 */
export function withDefault<T>(default_: T, maybe: Maybe<T>): T {
    return undefined != maybe && null != maybe ? maybe : default_;
}


/**
 * Map maybe value.
 * 
 * @param mapFn - function that will take unpacked value with type T and
 *                transform it into a value with type V.
 * @param maybe - Maybe value that we're mapping
 * @returns new value with type V as a Maybe type
 */
export function map<T, V>(mapFn:(val:T) => V, maybe: Maybe<T>): Maybe<V> {
    return undefined != maybe && null != maybe ? mapFn(maybe) : Nothing();
}
