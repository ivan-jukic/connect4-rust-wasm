module Page.Home exposing (Model, Msg, init, subs, update, view)

import Html exposing (Html, div, text)
import Html.Attributes exposing (class)
import Html.Events exposing (onClick)
import Html.Keyed
import Json.Encode as Encode
import List.Extra as List
import Ports


type Player
    = Player
    | Ai


playerToStr : Player -> String
playerToStr p =
    case p of
        Player ->
            "player"

        Ai ->
            "ai"


type Difficulty
    = Easy
    | Medium
    | Hard


difficultyToStr : Difficulty -> String
difficultyToStr d =
    case d of
        Easy ->
            "easy"

        Medium ->
            "medium"

        Hard ->
            "hard"


type alias Board =
    List (List Player)


maxCol : Int
maxCol =
    7


maxTokens : Int
maxTokens =
    6


{-| Init board with 7 empty lists, one for each column
-}
emptyBoard : Board
emptyBoard =
    [ [], [], [], [], [], [], [] ]


encodeBoard : Difficulty -> Player -> Board -> Encode.Value
encodeBoard difficulty player board =
    Encode.object
        [ ( "difficulty", Encode.string (difficultyToStr difficulty) )
        , ( "player", Encode.string (playerToStr player) )
        , ( "board"
          , Encode.list
                (Encode.list (playerToStr >> Encode.string))
                board
          )
        ]


dropToken : Int -> Player -> Board -> Result String Board
dropToken idx player board =
    let
        colHasSpace () =
            board
                |> List.getAt idx
                |> Maybe.map (List.length >> (>) maxTokens)
                |> Maybe.withDefault False
    in
    if idx <= maxCol then
        if colHasSpace () then
            Ok <|
                List.updateAt
                    idx
                    (\col -> col ++ [ player ])
                    board

        else
            Err "Column doesn't have enough space"

    else
        Err "Unknown board index"


type alias Model =
    { current : Player
    , difficulty : Difficulty
    , board : Board
    , err : Maybe String
    , winner : Maybe Player
    }


init : Model
init =
    { current = Player
    , difficulty = Medium
    , board = emptyBoard
    , err = Nothing
    , winner = Nothing
    }


type Msg
    = DropToken Int
    | Restart
    | AiMoveFinished ( Int, Bool )
    | HasPlayerWon Bool


subs : Model -> Sub Msg
subs model =
    Sub.batch
        [ Ports.aiMoveFinished AiMoveFinished
        , Ports.hasPlayerWon HasPlayerWon
        ]


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        DropToken col ->
            if Nothing /= model.winner then
                ( model, Cmd.none )

            else
                case ( model.current, dropToken col Player model.board ) of
                    ( Player, Ok newBoard ) ->
                        ( { model
                            | board = newBoard
                            , current = Ai
                            , err = Nothing
                          }
                        , Ports.checkPlayerWin (encodeBoard model.difficulty Player newBoard)
                        )

                    ( Player, Err err ) ->
                        ( { model | err = Just err }, Cmd.none )

                    ( Ai, _ ) ->
                        ( { model | err = Just "Waiting for AI to make move" }, Cmd.none )

        HasPlayerWon True ->
            ( { model | winner = Just Player }, Cmd.none )

        HasPlayerWon False ->
            ( model, Ports.runAiMove (encodeBoard model.difficulty Ai model.board) )

        Restart ->
            ( init, Cmd.none )

        AiMoveFinished ( col, isWinner ) ->
            case ( model.current, dropToken (col - 1) Ai model.board ) of
                ( Ai, Ok newBoard ) ->
                    ( { model
                        | board = newBoard
                        , current = Player
                        , err = Nothing
                        , winner =
                            if isWinner then
                                Just Ai

                            else
                                model.winner
                      }
                    , Cmd.none
                    )

                ( Ai, Err err ) ->
                    ( { model | err = Just <| "AI:" ++ err }, Cmd.none )

                ( Player, _ ) ->
                    ( { model | err = Just "Ai player cannot play now" }, Cmd.none )


view : Model -> Html Msg
view model =
    div [ class "page -home" ]
        [ div [ class "options" ]
            [ div [ class "opt -title" ]
                [ text "Connect4" ]

            -- Shows current player
            , case model.err of
                Just err ->
                    div [ class "opt -err" ]
                        [ div [ class "err-msg" ] [ text err ]
                        ]

                Nothing ->
                    div
                        [ class "opt -current"
                        , class <| "-" ++ playerToStr model.current
                        ]
                        [ text ("Playing // " ++ playerToStr model.current)
                        ]

            -- Menu with options
            , div [ class "opt -menu" ]
                [ Html.button [ class "item", onClick Restart ] [ text "Start again" ]
                , div [] [ text "//" ]
                , Html.button [ class "item" ] [ text "Easy" ]
                , Html.button [ class "item" ] [ text "Medium" ]
                , Html.button [ class "item" ] [ text "Hard" ]
                , div [] [ text "//" ]
                , Html.button [ class "item" ] [ text "Github" ]
                ]
            ]
        , div [ class "content" ]
            [ boardView model.board
            ]

        -- Winner
        , case model.winner of
            Just winner ->
                div [ class <| "winner -" ++ playerToStr winner ]
                    [ div [ class "msg" ]
                        [ Html.span [ class "label" ] [ text (playerToStr winner) ]
                        , text " has won!"
                        ]
                    , Html.button [ class "try-again", onClick Restart ] [ text "Try again" ]
                    ]

            Nothing ->
                text ""
        ]


boardView : Board -> Html Msg
boardView board =
    div [ class "board" ]
        (List.map
            (columnView board)
            (List.range 0 (maxCol - 1))
        )


columnView : Board -> Int -> Html Msg
columnView board colIdx =
    div
        [ class "column"
        , onClick (DropToken colIdx)
        ]
        [ Html.Keyed.node "div"
            [ class "tokens" ]
            (board
                |> List.getAt colIdx
                |> Maybe.withDefault []
                |> List.reverse
                |> List.indexedMap
                    (\rowIdx token ->
                        ( "token-"
                            ++ String.fromInt colIdx
                            ++ String.fromInt rowIdx
                        , div [ class ("token -" ++ playerToStr token) ] []
                        )
                    )
            )
        , div [ class "label" ]
            [ text (String.fromInt (colIdx + 1) ++ ". Col")
            ]
        ]
