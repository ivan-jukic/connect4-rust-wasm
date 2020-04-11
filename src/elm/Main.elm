module Main exposing (main)

import Browser exposing (Document, UrlRequest(..))
import Browser.Navigation as Navigation exposing (Key)
import Content
import Html exposing (div)
import Html.Attributes exposing (class)
import Routes exposing (Route(..), parseUrl)
import Url exposing (Url)


main : Program () Model Msg
main =
    Browser.application
        { init = init
        , view = view
        , update = update
        , subscriptions = always Sub.none
        , onUrlRequest = OnUrlRequest
        , onUrlChange = OnUrlChange
        }


type Msg
    = OnUrlRequest UrlRequest
    | OnUrlChange Url
    | ContentMsg Content.Msg


type alias Model =
    { route : Route
    , navigationKey : Key
    , content : Content.Model
    }


init : flags -> Url -> Key -> ( Model, Cmd Msg )
init _ initialUrl navigationKey =
    let
        initialRoute =
            parseUrl initialUrl
    in
    ( { route = initialRoute
      , navigationKey = navigationKey
      , content = setContent initialRoute Content.init
      }
    , Cmd.none
    )


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        -- How do we handle on url change?
        OnUrlChange newUrl ->
            let
                newRoute =
                    parseUrl newUrl
            in
            ( { model
                | route = newRoute
                , content = setContent newRoute model.content
              }
            , Cmd.none
            )

        OnUrlRequest urlRequest ->
            case urlRequest of
                Internal internUrl ->
                    ( model
                    , internUrl
                        |> Url.toString
                        |> Navigation.pushUrl model.navigationKey
                    )

                External extUrl ->
                    ( model, Navigation.load extUrl )

        ContentMsg subMsg ->
            let
                ( updated, subCmd ) =
                    Content.update subMsg model.content
            in
            ( { model | content = updated }
            , Cmd.map ContentMsg subCmd
            )


setContent : Route -> Content.Model -> Content.Model
setContent route =
    case route of
        Home ->
            Content.initHomePage

        NotFound ->
            Content.initNotFoundPage


view : Model -> Document Msg
view model =
    { title = "Connect4"
    , body =
        [ div
            [ class "app" ]
            [ Html.map ContentMsg <|
                Content.view model.content
            ]
        ]
    }
