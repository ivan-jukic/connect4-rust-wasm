module Content exposing
    ( Content(..)
    , Model
    , Msg
    , init
    , initHomePage
    , initNotFoundPage
    , update
    , view
    )

import Html exposing (Html, div)
import Html.Attributes exposing (class)
import Page.Home as HomePage
import Page.NotFound as NotFoundPage


type Content
    = ContentHome HomePage.Model
    | ContentNotFound


type Msg
    = HomePageMsg HomePage.Msg


type alias Model =
    { content : Content
    }


init : Model
init =
    { content = ContentHome HomePage.init
    }


initHomePage : Model -> Model
initHomePage model =
    { model | content = ContentHome HomePage.init }


initNotFoundPage : Model -> Model
initNotFoundPage model =
    { model | content = ContentNotFound }


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case ( msg, model.content ) of
        ( HomePageMsg subMsg, ContentHome m ) ->
            let
                ( updated, subCmd ) =
                    HomePage.update subMsg m
            in
            ( { model | content = ContentHome updated }
            , Cmd.map HomePageMsg subCmd
            )

        _ ->
            ( model, Cmd.none )


view : Model -> Html Msg
view model =
    div [ class "content" ]
        [ case model.content of
            ContentHome m ->
                Html.map HomePageMsg <|
                    HomePage.view m

            ContentNotFound ->
                NotFoundPage.view
        ]
