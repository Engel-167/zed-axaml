; The outline lists the root, every named element (<Button x:Name="SaveButton"> shows as Button SaveButton),
; resources (x:Key) and styles (Selector). Turn on the language server's outline for the full tree:
; "languages": { "AXAML": { "document_symbols": "on" } }

(element
  (STag
    (Name) @name
    (Attribute
      (Name) @_attr
      (AttValue) @context))
  (#match? @_attr "^((x:)?Name|(x:)?Key|Selector)$")) @item

(element
  (EmptyElemTag
    (Name) @name
    (Attribute
      (Name) @_attr
      (AttValue) @context))
  (#match? @_attr "^((x:)?Name|(x:)?Key|Selector)$")) @item

(document
  (element
    (STag (Name) @name)) @item)
