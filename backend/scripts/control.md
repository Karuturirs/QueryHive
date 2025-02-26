 About serving controls

You can apply serving controls to serving configs to customize how search treats search queries and returns results. A serving control is a rule consisting of a condition-action pair, where the condition dictates when the serving control will execute, and the action specifies what behavior the serving control will enact.

You can create a serving control by using the API Control.create method.

If you don't want to use the API, you can choose console for creating serving controls in Vertex AI Search for commerce.
Available serving controls

The following serving controls are available:

    Boost/bury: Affects result ranking and order in the returned result list. Available for search and recommendations.
    Filter: Removes results that don't pass the filter from the returned result list. Available for search only.
    Redirect: Redirects your users to a specific page depending on the search query. Available for search only.
    Pinning: Exact position in the results is specified for a certain product.
    Linguistic: Customizes search query linguistics. Available for search only. Several types of linguistic controls are available:
        Synonym: Expands considered synonyms for a search query.
        One-way synonym: Expands considered synonyms unidirectionally for specific terms.
        Ignore: Prevents a term from being used in searches.
        Do not associate: Prevent terms from being used in searches when specific terms appear.
        Replacement: Replaces terms in the search query.

Control conditions

Control conditions dictate when a serving control will execute.

Control condition fields can be query terms, time ranges, or both. Some control types allow multiple condition fields, or don't allow any.

The condition fields available:

    Query terms: Triggered when the term appears in the search query.
        A full match requires the entire search query to match the query term.
        Multiple query terms can be specified. Triggers as long as one of the query terms appears in the search query.
    Active time range: Triggered when the date of the search query is in the time range.
        Multiple time ranges can be specified. Triggers as long as date of query is within the time range (inclusive).

The condition fields that you specify determine whether the control will be applied.

    Multiple condition fields are combined using AND. This means that if you specify both time range and query terms, both condition fields need to be triggered for the control to apply.
    Multiple condition sub-fields are combined using OR. This means that if you have multiple query terms the query terms will be triggered if any one query term matches. If you have multiple time ranges, any one time range that matches will trigger the control.
    No condition fields specified mean the control always applies. However, some controls require a field to be defined.

For more about condition settings, see the Controls.condition API reference.
Control actions

A control action specifies what behavior the serving control will enact if the conditions are met during a search.

What kind of action you can specify depends on the type of serving control you create. For example, the action for a boost/bury control is to apply a boost/bury value to products that the filter you specify, while the action for a one-way synonym control is to apply an associated term that you specify.
Boost/bury controls

Boost/bury controls enable you to show certain search results as higher or lower in ranking.

You can create a boost/bury control for search or recommendations. Boost/bury for recommendations is in Public Preview.

When creating a boost/bury control, you can use filter expressions to specify the conditions based on Product fields.

    For search filters, use the filter expression syntax documented in Filter and order results.
    For recommendations filters, use the filter expression syntax documented in Filter recommendations.

You can then apply a boost value between -1.0 and 1.0 to indicate how much to boost or bury product results matching those conditions. A positive value boosts the results, and a negative value buries them.

Setting a high boost strength gives the item a large promotion, but doesn't necessarily mean that the boosted item will be the top result at all times. Results that are significantly more relevant to the search query can still trump heavily favored but irrelevant items. Likewise, setting the boost strength to -1.0 would give the item a large demotion, but results that are deeply relevant might still be shown.

As an example of using boost/bury, you could prioritize cheaper products and deprioritize the expensive ones.

Control conditions differ between search and recommendations boost/bury controls:

    Search: You can set query terms and applicable time ranges as the control conditions.
    Recommendations: Control conditions aren't available. The control always applies.

As a control action, specify a filter for products to boost or bury, and set the boost/bury value.

To create a boost/bury control, see Create a new serving control.

For more about boost/bury control settings, see the Controls.BoostAction API reference.
Filter controls

With filter controls, you can dynamically add predefined filters based on a specific search request.

You can use filter expressions based on Product fields. See Filter and order results for the filter expression syntax.

You can set query terms and applicable time ranges as the control conditions. As a control action, specify a filter to apply at query time.

For example, given the query blue shoes, you can use a filter control to automatically filter search results on the color blue. You can also use filter controls to prevent certain results from being returned to shoppers.

To create a filter control, see Create a new serving control.

For more about filter control settings, see the Controls.FilterAction API reference.
Redirect controls

You can use a redirect control to redirect your shoppers to different pages based on their intent, instead of only showing them search results.

You can set query terms and applicable time ranges as the control conditions. As a control action, specify a redirect URI to redirect to if the conditions are matched.

For example, you could create a redirect control so that during a promotion for a the product gShoe, queries with running shoes or sports shoes redirects to the gShoe product page. Another case for using redirect controls would be to redirect shoppers to a specific page if they search for a term that is not relevant to your site, such as a search for FAQ redirecting the user to the actual Frequently Asked Questions page on your site instead of showing search results.

To create a redirect control, see Create a new serving control.

For more about redirect control settings, see the Controls.RedirectAction API reference.
Pinning controls

Pinning controls let you specify an exact position in the results you want a certain product to appear.

You can create a pinning control for search or browse. It is not supported for recommendations.

When creating a pinning control, you can use filter expressions to specify the conditions based on Product fields.

As a control action, add a Rule to your search or browse Condition which will be the action field pin_action.

You can then apply a pin value between [1,120] to indicate the fixed position to pin the results to matching those conditions given.

    Rule Condition: Must specify non-empty [Condition.query_terms][] (for search only) or [Condition.page_categories][] (for browse only), but not both.
    Action Input: [pin_position, product_id] pairs are mapped (the input position can be a value from 1 to 120). The maximum size is the maximum request page size. 10 is the number of allowed pairs in the pin map.
    Action Result: Pin products with matching IDs to the position specified in the final result order. To name an example, suppose the query is shoes, the [Condition.query_terms][] is "shoes" and the pin_map has {1, pid1}. The product with pid1 is pinned to the top position in the final results.

Note: A query with an active pin experiences 100ms of added latency.
Enabling pins and applied rules

When a pinning control is enabled:

    Products with product IDs which match a pin appear in the final response in the exact position specified by the control.
    Pins to the second page or later (higher page number) are not allowed, that is, any pin that is set to a position higher than the request page_size is ignored.
    A maximum of 10 products can be pinned with one control.
    The product_id must be the ID of an existing product in the catalog.

The product is always recalled in every search and appears on the page unless:

    Filters and sorting are applied. These prevent pins from appearing.
    The sort order (such as having the search results sorted by price) is not the default.

Pin behavior

    If a product is pinned, it will ignore any boosts or buries from the request or from other controls.
    If a product is pinned to multiple positions, the most recently updated control takes precedence.
    Two products cannot be pinned to the same position inside the same pinning control (pin map).
        If multiple controls match the same query and each of those have a different pin for the same product ID, the [pin_pos, p_id] pair from the most recently updated control takes precedence.

Caution: Adding pinning controls to many common queries can adversely impact your total latency. Use sparingly and only when necessary.
Linguistic controls

You can create additions or overrides to how words are treated for certain queries.
Synonym controls

Setting two words as synonyms is a linguistic control that associates two words.

Synonym controls add additional context to a search query. They don't force a result to be included in the search results, but they can help the system include additional products in the search results, making it more likely that a given result is included. In other words, synonym controls can encourage the search result to consider more options, but ultimately the search result will be dependent on scoring.

For example, if you want search results for running shoes to also include sport shoes, create a linguistic synonym control. The condition is that running shoes is entered as the search term. The action is to include the synonym sport shoes with that search. So when a shopper on your site searches running shoes, search finds that match in the linguistic control you created, expanding it to include sport shoes when it returns search results to the shopper.

To create a synonym control, see Create a new serving control.
Two-way synonym controls

Use two-way synonym controls to link several terms together so that search treats them the same during searches.

You can set query terms and applicable time ranges as the control conditions. You don't need to set a separate control action; if a term you specified is used as a query, the control action is to use other terms you specified as the synonyms.

For example, you could set a two-way synonym control that associates the terms dish towel and kitchen towel as synonyms. When a shopper on your site enters kitchen towel as a query, search can then expand the query to include results for dish towel and kitchen towel.

To create a synonym control, see Create a new serving control.

Synonyms don't change the original query. For example, if queries A and B are a two-way synonym, expect the following effect:

    Query A results returned: Set A (with no synonym rule applied) and some of set B. However, the results might be less than the combination of A and B.

    Query B results returned: Set B and some of A, but possibly less than the sum of result sets A and B.

For more about two-way synonym control settings, see the Controls.TwowaySynonymsAction API reference.
One-way synonym controls

One-way synonym controls expand query terms to link terms together unidirectionally.

You can set query terms and applicable time ranges as the control conditions. As a control action, specify the terms to use as one-way synonyms.

For example, you could set a one-way synonym control that expands searches for the term rose to include the term pink. Because it is a one-way synonym, searches for the term pink don't expand to include the term rose.

To create a one-way synonym control, see Create a new serving control.

Note that just as with two-way synonyms, the original query is executed and the synonyms are provided as hints to that query. Synonyms for entirely different words may only result in small numbers of the synonym terms being included in the results. The preceding example returns pink items with an emphasis on pink roses, but a synonym rule expanding searches for the term dog to include cat returns mostly items with dogs, plus some with cats.

For more about one-way synonym control settings, see the Controls.OnewaySynonymsAction API reference.
Synonyms don't provide identical results

Search results for two synonyms aren't always identical.

For example, if you set laptop bags and luggage as two-way synonyms, the system might already associate suitcase with luggage. So, when a user searches for luggage, Vertex AI Search for commerce returns results about luggage, suitcase, and laptop bags. However, when you search for laptop bags, Vertex AI Search for commerce only adds luggage to the synonyms. So the results might not contain suitcase.
Ignore controls

Ignore controls prevent search from using certain query terms during searches. They mark ignored terms in a query as not important, but does not exclude them from the query entirely.

To completely remove a search term from a query, use a replacement control. While a filter control affects which results are shown, a replacement control is a more robust way to prevent Vertex AI Search for commerce from returning results for a specific term.

An ignore control doesn't guarantee that results for the ignored term won't be returned in a Vertex AI Search for commerce results. For example, an ignore control for the word oil could still return oil paints for a query of oil paints. The query will be passed as paints, and there may be many oil paints matched, but the result set will be larger as the search is for paints. This type of control might be useful if oil paints were a popular query on your site and you sell many different types of paints, but not many oil-based paints.

You can set query terms and applicable time ranges as the control conditions. You don't need to set a separate control action; if a term you specified is used as a query, the control action is to ignore that term.

For example, you could create a control that ignores query terms that use offensive language.

To create an ignore control, see Create a new serving control.

For more about ignore control settings, see the Controls.IgnoreAction API reference.
Do-not-associate controls

Do-not-associate controls prevent query terms from being queried together during searches with other terms that you specify.

You can set query terms and applicable time ranges as the control conditions. As a control action, specify the terms that shouldn't be associated with the query terms.

For example, you could create a control that prevents a brand name (such as gShoe) from being grouped with the term cheap and poor quality in a query, so that if a shopper searches for poor quality cheap gShoe, search searches only for gShoe.

If a relevant result for the query term also contains a term specified as do not associate, that relevant result might still be returned. To prevent this from happening entirely, use a filter control.

To create a do-not-associate control, see Create a new serving control.

For more about do-not-associate control settings, see the Controls.DoNotAssociateAction API reference.
Replacement controls

Replacement controls replace one or more given query terms with a different term that you specify. You can specify multiple terms that can be mapped to single term (but not vice versa).

You can set query terms and applicable time ranges as the control conditions. As a control action, specify the term that should be used as a replacement.

For example, you could create a control that replaces nicknames for a brand with the full brand name that is more commonly used in product descriptions.

To create a replacement control in the Search for commerce console, see Create a new serving control.

For more about replacement control settings, see the Controls.ReplacementAction API reference.