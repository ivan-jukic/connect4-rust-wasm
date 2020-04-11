/// Elm app is defined as an interface that exposes port functions.
export interface ElmApp {
    ports: Ports;
}


/// Type definitions for different elm ports. 
export interface Ports {

    // Log port
    log: LogSub;
}


/// Subscription interface for the logging port!
type LogFn = (val: string) => void;

interface LogSub {
    subscribe: (handler: LogFn) => void;
}
