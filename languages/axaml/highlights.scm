; AXAML highlighting on top of tree-sitter-xml. Names are single tokens in this grammar, so the
; AXAML-specific cases (xmlns, x: directives, Owner.Property, markup extensions) are told apart by
; predicates. The predicates are mutually exclusive, so the result does not depend on pattern order.

;; Prolog / processing instructions

"xml" @keyword
[ "version" "encoding" "standalone" ] @property
(EncName) @string.special
(VersionNum) @number
[ "yes" "no" ] @boolean
(PI) @embedded
(PI (PITarget) @keyword)

;; Element names

; <Button>, <vm:MyControl>
((STag (Name) @tag) (#not-match? @tag "\\."))
((EmptyElemTag (Name) @tag) (#not-match? @tag "\\."))
((ETag (Name) @tag) (#not-match? @tag "\\."))

; Property elements: <Grid.RowDefinitions>, <UserControl.Styles>
((STag (Name) @property) (#match? @property "\\."))
((EmptyElemTag (Name) @property) (#match? @property "\\."))
((ETag (Name) @property) (#match? @property "\\."))

;; Attribute names

; xmlns, xmlns:vm
((Attribute (Name) @keyword) (#match? @keyword "^xmlns(:|$)"))

; x:Class, x:DataType, x:Name, d:DesignWidth, mc:Ignorable ...
((Attribute (Name) @keyword)
 (#match? @keyword "^[A-Za-z_][A-Za-z0-9_]*:[^.]*$")
 (#not-match? @keyword "^xmlns:"))

; Attached properties: Grid.Row, ScrollViewer.HorizontalScrollBarVisibility, local:Behavior.Value
((Attribute (Name) @attribute) (#match? @attribute "\\.") (#not-match? @attribute "^xmlns"))

; Plain properties and events: Text, Margin, Click
((Attribute (Name) @property) (#not-match? @property "[.:]") (#not-match? @property "^xmlns$"))

;; Attribute values

; Markup extensions: "{Binding Title}", "{StaticResource Key}"  ("{}" is the literal escape)
((Attribute (AttValue) @string.special)
 (#match? @string.special "^[\"']\\s*\\{[^}]"))

; Everything else
((Attribute (AttValue) @string)
 (#not-match? @string "^[\"']\\s*\\{[^}]"))

;; Entities and references

(EntityRef) @constant
((EntityRef) @constant.builtin
 (#any-of? @constant.builtin "&amp;" "&lt;" "&gt;" "&quot;" "&apos;"))
(CharRef) @constant

;; Punctuation

[ "<?" "?>" "<!" "]]>" "<" ">" "</" "/>" ] @punctuation.delimiter
[ "\"" "'" ] @punctuation.delimiter
"=" @operator

;; Text, CDATA, comments

(CharData) @text.literal
(CDSect (CDStart) @punctuation.special (CData) @text.literal "]]>" @punctuation.special)
(Comment) @comment
