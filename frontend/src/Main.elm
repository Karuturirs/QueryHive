module Main exposing (..)

import Browser
import Dict exposing (Dict)
import Html exposing (Html, button, div, input, text, textarea, ul, li, span, h2, h3, p)
import Html.Attributes exposing (placeholder, value, class)
import Html.Events exposing (onClick, onInput)
import Http
import Json.Decode as Decode
import Json.Encode as Encode


-- MODEL
type alias Document =
    { id : String
    , title : String
    , path : String
    , tags : List String
    , created_at : String
    }

type alias DocumentForm =
    { title : String
    , content : String
    , path : String
    , tags : String
    }

type alias Model =
    { chatInput : String
    , documentForm : DocumentForm
    , documents : List Document
    , folderStructure : Dict String (List Document)
    , response : Maybe String
    }

initialModel : Model
initialModel =
    { chatInput = ""
    , documentForm = { title = "", content = "", path = "", tags = "" }
    , documents = []
    , folderStructure = Dict.empty
    , response = Nothing
    }

-- MESSAGES

type Msg
    = UpdateChatInput String
    | UpdateTitle String
    | UpdateContent String
    | UpdatePath String
    | UpdateTags String
    | SubmitDocument
    | DocumentAdded (Result Http.Error String)
    | FetchDocuments
    | DocumentsFetched (Result Http.Error (List Document))


-- UPDATE FUNCTION

update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        UpdateChatInput input ->
            ( { model | chatInput = input }, Cmd.none )

        UpdateTitle title ->
            ( { model | documentForm = {  model.documentForm |  title = title } }, Cmd.none )

        UpdateContent content ->
            ( { model | documentForm = {  content = content } }, Cmd.none )

        UpdatePath path ->
            ( { model | documentForm = {  path = path } }, Cmd.none )

        UpdateTags tagStr ->
            ( { model | documentForm = {  tags = tagStr } }, Cmd.none )

        SubmitDocument ->
            let
                jsonBody =
                    Http.jsonBody (encodeDocument model.documentForm)
            in
            ( model
            , Http.post
                { url = "http://localhost:3001/documents"
                , body = jsonBody
                , expect = Http.expectString DocumentAdded
                }
            )

        DocumentAdded (Ok response) ->
            ( { model | response = Just response }, Cmd.batch [ Cmd.none, fetchDocuments ] )

        DocumentAdded (Err _) ->
            ( { model | response = Just "Failed to add document" }, Cmd.none )

        FetchDocuments ->
            ( model, fetchDocuments )

        DocumentsFetched (Ok docs) ->
            ( { model | documents = docs, folderStructure = buildFolderStructure docs }, Cmd.none )

        DocumentsFetched (Err _) ->
            ( { model | response = Just "Failed to fetch documents" }, Cmd.none )


-- HTTP REQUESTS

fetchDocuments : Cmd Msg
fetchDocuments =
    Http.get
        { url = "http://localhost:3001/documents"
        , expect = Http.expectJson DocumentsFetched (Decode.list decodeDocument)
        }


-- JSON ENCODING & DECODING

encodeDocument : DocumentForm -> Encode.Value
encodeDocument doc =
    Encode.object
        [ ( "title", Encode.string doc.title )
        , ( "content", Encode.string doc.content )
        , ( "path", Encode.string doc.path )
        , ( "tags", Encode.list Encode.string (String.split "," doc.tags) )
        ]

decodeDocument : Decode.Decoder Document
decodeDocument =
    Decode.map5 Document
        (Decode.field "id" Decode.string)
        (Decode.field "title" Decode.string)
        (Decode.field "path" Decode.string)
        (Decode.field "tags" (Decode.list Decode.string))
        (Decode.field "created_at" Decode.string)


-- VIEW

view : Model -> Html Msg
view model =
    div []
        [ div [ class "sidebar" ]
            [ h2 [] [ text "Indexed Documents" ]
            , viewDocuments model.documents
            ]
        , div [ class "main-content" ]
            [ div [ class "chat-box" ]
                [ textarea
                    [ placeholder "Enter your message..."
                    , value model.chatInput
                    , onInput UpdateChatInput
                    ]
                    []
                , button [ onClick SubmitDocument ] [ text "Submit" ]
                ]
            , div [ class "document-form" ]
                [ input [ placeholder "Title", value model.documentForm.title, onInput UpdateTitle ] []
                , textarea [ placeholder "Content", value model.documentForm.content, onInput UpdateContent ] []
                , input [ placeholder "Path", value model.documentForm.path, onInput UpdatePath ] []
                , input [ placeholder "Tags (comma separated)", value model.documentForm.tags, onInput UpdateTags ] []
                , button [ onClick SubmitDocument ] [ text "Attach Document" ]
                ]
            ]
        ]

viewDocuments : List Document -> Html Msg
viewDocuments documents =
    ul []
        (List.map viewDocument documents)

viewDocument : Document -> Html Msg
viewDocument doc =
    li []
        [ h3 [] [ text doc.title ]
        , p [] [ text ("Path: " ++ doc.path) ]
        , p [] [ text ("Tags: " ++ String.join ", " doc.tags) ]
        , p [] [ text ("Created at: " ++ doc.created_at) ]
        ]

-- FILE STRUCTURE BUILDING

type alias FolderStructure =
    Dict String (List Document)

buildFolderStructure : List Document -> FolderStructure
buildFolderStructure docs =
    List.foldl (\doc acc -> Dict.update doc.path (Maybe.withDefault [] >> (::) doc >> Just) acc) Dict.empty docs


-- RENDERING FOLDERS & FILES

renderFolderStructure : FolderStructure -> Html Msg
renderFolderStructure folderStructure =
    ul []
        (Dict.toList folderStructure
            |> List.map (\( folderPath, files ) -> renderFolder folderPath files)
        )

renderFolder : String -> List Document -> Html Msg
renderFolder path files =
    li []
        [ span [] [ text ("📂 " ++ path) ]
        , ul [] (List.map renderFile files)
        ]

renderFile : Document -> Html Msg
renderFile doc =
    li [] [ text ("📄 " ++ doc.title) ]


-- MAIN

main : Program () Model Msg
main =
    Browser.element
        { init = \_ -> ( initialModel, fetchDocuments )
        , update = update
        , view = view
        , subscriptions = \_ -> Sub.none
        }