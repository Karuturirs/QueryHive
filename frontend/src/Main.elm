module Main exposing (..)

import Browser
import Dict exposing (Dict)
import Html exposing (Html, button, div, input, text, textarea, ul, li, span)
import Html.Attributes exposing (placeholder, value)
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

type alias Model =
    { document : DocumentForm
    , documents : List Document
    , folderStructure : Dict String (List Document)
    , response : Maybe String
    }

type alias DocumentForm =
    { title : String
    , content : String
    , path : String
    , tags : String
    }

initialModel : Model
initialModel =
    { document = { title = "", content = "", path = "", tags = "" }
    , documents = []
    , folderStructure = Dict.empty
    , response = Nothing
    }


-- MESSAGES

type Msg
    = UpdateTitle String
    | UpdateContent String
    | UpdatePath String
    | SubmitDocument
    | DocumentAdded (Result Http.Error String)
    | FetchDocuments
    | DocumentsFetched (Result Http.Error (List Document))


-- UPDATE FUNCTION

update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        UpdateTitle title ->
            ( { model | document = { model.document | title = title } }, Cmd.none )

        UpdateContent content ->
            ( { model | document = { model.document | content = content } }, Cmd.none )

        UpdatePath path ->
            ( { model | document = { model.document | path = path } }, Cmd.none )

         UpdateTags tagStr ->
            let
                tags = String.split "," tagStr
            in
            ( { model | document = { model.document | tags = tags } }, Cmd.none )

        SubmitDocument ->
            let
                jsonBody =
                    Http.jsonBody (encodeDocument model.document)
            in
            ( model
            , Http.post
                { url = "http://localhost:8080/documents"
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
        { url = "http://localhost:8080/documents"
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
        [ div []
            [ input [ placeholder "Title", onInput UpdateTitle ] []
            , textarea [ placeholder "Content", onInput UpdateContent ] []
            , input [ placeholder "Path (e.g., /documents/project1/file1.md)", onInput UpdatePath ] []
            , input [ placeholder "Tags (comma-separated)", onInput UpdateTags ] []
            , button [ onClick SubmitDocument ] [ text "Add Document" ]
            ]
        , case model.response of
            Just msg -> div [] [ text msg ]
            Nothing -> text ""
        , div [] [ text "File Structure:", renderFolderStructure model.folderStructure ]
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