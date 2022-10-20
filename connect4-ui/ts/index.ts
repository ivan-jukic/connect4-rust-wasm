// @ts-ignore
import { Elm } from "../src/Main.elm";

// Import required files built by the wasm-pack!
//
// Parcel2 does not know what to do with the *.wasm files, so we're just using
// it to copy the file into the build folder, and get the file path.
//
// We can then use this file path in the next step, where we load the generated
// JS file, and use the provided API to initialise the WASM module. To do this
// we need to use the `init` function which will take the path to the WASM file
// as an arg.
//
// @ts-ignore
import wasmPath from "url:./../../.build-wasm/connect4_bg.wasm";
import init, { process_data_for_ai, process_data_for_player } from "./../../.build-wasm/connect4.js";

declare global {
    interface Window {
        connect4_ai_move: (c: number, w: boolean) => void;
        connect4_player_win: (w: boolean) => void;
        connect4_err: (e: string) => void;
    }
}

document.addEventListener("DOMContentLoaded", async () => {
    await init(wasmPath);
    const elmApp = Elm.Main.init({
        node: document.querySelector("main")
    });

    window.connect4_ai_move = function (col: number, isWinner: boolean) {
        if (elmApp.ports.aiMoveFinished) {
            elmApp.ports.aiMoveFinished.send([col, isWinner]);
        }
    }

    window.connect4_player_win = function (isWinner: boolean) {
        if (elmApp.ports.hasPlayerWon) {
            console.log("process data for player 3", isWinner);
            elmApp.ports.hasPlayerWon.send(isWinner);
        }
    }

    window.connect4_err = function (err: string) {
        console.error("Connect4 WASM error: ", err);
    }

    /// Init ports
    if (elmApp.ports.runAiMove) {
        elmApp.ports.runAiMove.subscribe((state: object) => {
            process_data_for_ai(JSON.stringify(state));
        });
    }

    if (elmApp.ports.checkPlayerWin) {
        elmApp.ports.checkPlayerWin.subscribe((state: object) => {
            process_data_for_player(JSON.stringify(state));
        });
    }
});
