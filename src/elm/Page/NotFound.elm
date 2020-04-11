module Page.NotFound exposing (view)

import Html exposing (Html, div, text)
import Html.Attributes exposing (class)


view : Html msg
view =
    div [ class "page -not-found" ]
        [ div [] [ text "This is not the page you're looking for..." ]
        ]
