port module Ports exposing
    ( aiMoveFinished
    , checkPlayerWin
    , hasPlayerWon
    , runAiMove
    )

import Json.Encode as Encode


port runAiMove : Encode.Value -> Cmd msg


port aiMoveFinished : (( Int, Bool ) -> msg) -> Sub msg


port checkPlayerWin : Encode.Value -> Cmd msg


port hasPlayerWon : (Bool -> msg) -> Sub msg
