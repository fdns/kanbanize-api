# BoardSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**size_type** | Option<**i32**> | Controls how the card sizes are entered and displayed. If set to 0, the card size is an input field, which allows any numeric value. If set to 1, the card size is a dropdown list with T-shirt sizes like S, M, L, XL, etc. If set to 2, the card size is a dropdown list with the Fibonacci numbers. | [optional]
**allow_exceeding** | Option<**i32**> | Controls whether exceeding the WIP limits on the board is allowed. If set to 0, the limits can always be exceeded. If set to 1, the limits can be exceeded only with an explanation. If set to 2, the limits cannot be exceeded. | [optional]
**autoarchive_cards_after** | Option<**i32**> | Controls number of days before automatically moving all cards from Ready to Archive to Permanent Archive. | [optional]
**limit_type** | Option<**i32**> | Controls the measurement unit for work limits on the board. When limit type is 0, the board limits are calculated based on cards count. When limit type is 1, the board limits are calculated based on the size of the cards, cards without set size are counted as 1. | [optional]
**allow_repeating_custom_card_ids** | Option<**i32**> | Controls whether repeating custom card ids are allowed on the board. When the value is 0, the custom ids of the cards have to be unique on the board. When the value is 1 repeating custom card ids are allowed. | [optional]
**is_discard_reason_required** | Option<**i32**> | Controls whether providing a discard reason is required when discarding a card. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


