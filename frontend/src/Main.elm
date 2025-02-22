module Main exposing (..)

import Browser
import Html exposing (Html, button, div, input, text, textarea, form)
import Html.Attributes exposing (placeholder, value, type_, class, style)
import Html.Events exposing (onClick, onInput, onSubmit)
import Http
import Json.Decode as Decode
import Json.Encode as Encode


-- MODEL

type alias Model =
    { chatInput : String
    , messages : List String
    , darkMode : Bool
    }


init : () -> ( Model, Cmd Msg )
init _ =
    ( { chatInput = ""
      , messages = []
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
            ( { model | messages = model.messages ++ [ "You: " ++ model.chatInput ], chatInput = "" }
            , Http.post
                { url = "http://localhost:3001/chat"
                , body = Http.jsonBody jsonBody
                , expect = Http.expectString DataSent
                }
            )

        DataSent (Ok response) ->
            ( { model | messages = model.messages ++ [ "AI: " ++ response ] }, Cmd.none )

        DataSent (Err _) ->
            ( { model | messages = model.messages ++ [ "AI: Failed to send data" ] }, Cmd.none )
            
        ToggleDarkMode ->
            ( { model | darkMode = not model.darkMode }, Cmd.none )

-- VIEW

view : Model -> Html Msg
view model =
    div [ style "background-color" (if model.darkMode then "#121212" else "#ffffff")
        , style "color" (if model.darkMode then "#ffffff" else "#000000")
        , style "padding" "20px"
        , style "border-radius" "10px"
        , style "max-width" "600px"
        , style "margin" "0 auto"
        ]
        [ button [ onClick ToggleDarkMode, style "margin-bottom" "20px" ] [ text "Toggle Dark Mode" ]
        , div [ style "border" "1px solid #ccc"
              , style "border-radius" "10px"
              , style "padding" "20px"
              , style "background-color" (if model.darkMode then "#333333" else "#f9f9f9")
              , style "max-height" "400px"
              , style "overflow-y" "auto"
              , style "margin-bottom" "20px"
              ]
            [ div []
                (List.map (\msg -> div [ style "margin-bottom" "10px" ] [ text msg ]) model.messages)
            , form [ onSubmit SendData ]
                [ textarea
                    [ placeholder "Enter your message..."
                    , value model.chatInput
                    , onInput UpdateChatInput
                    , style "width" "100%"
                    , style "height" "100px"
                    , style "margin-bottom" "10px"
                    ]
                    []
                , button [ type_ "submit", style "display" "block", style "width" "100%" ] [ text "Send" ]
                ]
            ]
        ]

-- MAIN

main =
    Browser.element
        { init = init
        , update = update
        , view = view
        , subscriptions = \_ -> Sub.none
        }