module Main exposing (..)

import Browser
import Html exposing (Html, button, div, input, text, textarea, form)
import Html.Attributes exposing (placeholder, value, type_ , class)
import Html.Events exposing (onClick, onInput, onSubmit)
import Http
import Json.Decode as Decode
import Json.Encode as Encode


-- MODEL

type alias Model =
    { chatInput : String
    , response : Maybe String
    , darkMode : Bool
    }


init : () -> ( Model, Cmd Msg )
init _ =
    ( { chatInput = ""
      , response = Nothing
      , darkMode = True
      }
    , Cmd.none
    )


-- MESSAGES

type Msg
    = UpdateChatInput String
    | Submit
    | SendData
    | DataSent (Result Http.Error String)
    | ToggleDarkMode


-- UPDATE

update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        UpdateChatInput input ->
            ( { model | chatInput = input }, Cmd.none )

        Submit ->
            ( model, Cmd.none )

        SendData ->
            let
                jsonBody =
                    Encode.object
                        [ ( "chatInput", Encode.string model.chatInput ) ]
            in
            ( model
            , Http.post
                { url = "http://localhost:3001/chat"
                , body = Http.jsonBody jsonBody
                , expect = Http.expectString DataSent
                }
            )

        DataSent (Ok response) ->
            ( { model | response = Just response }, Cmd.none )

        DataSent (Err _) ->
            ( { model | response = Just "Failed to send data" }, Cmd.none )
            
        ToggleDarkMode ->
            ( { model | darkMode = not model.darkMode }, Cmd.none )

-- VIEW

-- VIEW

view : Model -> Html Msg
view model =
    div[][
        form [ onSubmit SendData ]
                [ textarea
                    [ placeholder "Enter your message..."
                    , value model.chatInput
                    , onInput UpdateChatInput
                    ]
                    []
                , button [ type_ "submit" ] [ text "Send" ]
                ]
            , case model.response of
                Just response ->
                    div [] [ text response ]

                Nothing ->
                    text ""
    ]

-- MAIN

main =
    Browser.element
        { init = init
        , update = update
        , view = view
        , subscriptions = \_ -> Sub.none
        }