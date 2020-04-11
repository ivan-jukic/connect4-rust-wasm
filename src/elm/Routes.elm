module Routes exposing (Route(..), parseUrl)

import Url exposing (Url)
import Url.Parser as Url


type Route
    = Home
    | NotFound


parseUrl : Url -> Route
parseUrl =
    Maybe.withDefault NotFound
        << Url.parse
            (Url.oneOf
                [ Url.map Home Url.top
                ]
            )
