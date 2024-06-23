// Generated file; do not edit. See `schema-gen` crate.

use common::{serde::{Serialize, Deserialize}, strum::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
#[strum(serialize_all = "camelCase", crate = "common::strum")]
pub enum NodeProperty {
    About,
    Abstract,
    Address,
    AddressCountry,
    AddressLocality,
    AddressRegion,
    AdmonitionType,
    Affiliations,
    AlternateNames,
    Amounts,
    Arguments,
    Assignee,
    Author,
    Authors,
    AutoExec,
    AvailableLanguages,
    Bitrate,
    Brands,
    Caption,
    CellType,
    Cells,
    CharacterCount,
    CharacterPercent,
    Chars,
    CitationIntent,
    CitationMode,
    CitationPrefix,
    CitationSuffix,
    Cite,
    ClaimType,
    ClassList,
    Clauses,
    Code,
    CodeLocation,
    CodeRepository,
    CodeSampleType,
    ColumnSpan,
    Columns,
    CommentAspect,
    Comments,
    CompilationDigest,
    CompilationMessages,
    ContactPoints,
    Contains,
    Content,
    ContentSize,
    ContentUrl,
    Contributors,
    CostWeight,
    Css,
    DateAccepted,
    DateCreated,
    DateEnd,
    DateModified,
    DatePublished,
    DateReceived,
    DateStart,
    Default,
    Departments,
    DependantNode,
    DependantRelation,
    DependenciesDigest,
    DependenciesFailed,
    DependenciesStale,
    DependencyNode,
    DependencyRelation,
    DeriveAction,
    DeriveFrom,
    DeriveItem,
    DerivedFrom,
    Description,
    Editors,
    Emails,
    EmbedUrl,
    EndColumn,
    EndLine,
    EndPosition,
    ErrorType,
    ExclusiveMaximum,
    ExclusiveMinimum,
    ExecutionActor,
    ExecutionCount,
    ExecutionDependants,
    ExecutionDependencies,
    ExecutionDigest,
    ExecutionDuration,
    ExecutionEnded,
    ExecutionMessages,
    ExecutionPure,
    ExecutionRequired,
    ExecutionStatus,
    ExecutionTags,
    FamilyNames,
    Feedback,
    Format,
    FundedBy,
    FundedItems,
    Funders,
    Genre,
    GivenNames,
    HideSuggestions,
    Hint,
    HonorificPrefix,
    HonorificSuffix,
    Id,
    Identifiers,
    Images,
    InstructionType,
    IsActive,
    IsChecked,
    IsDisabled,
    IsFolded,
    IsGlobal,
    IsInvisible,
    IsPartOf,
    Issns,
    IssueNumber,
    Item,
    ItemReviewed,
    ItemType,
    ItemTypes,
    Items,
    ItemsNullable,
    ItemsValidator,
    Iterations,
    JobTitle,
    Keys,
    Keywords,
    Label,
    LabelAutomatically,
    LabelType,
    LastModified,
    LegalName,
    Length,
    Level,
    Licenses,
    Logo,
    Maintainers,
    MathLanguage,
    Mathml,
    MaxItems,
    MaxLength,
    Maximum,
    MediaType,
    MemberOf,
    Members,
    Message,
    Messages,
    MinItems,
    MinLength,
    Minimum,
    Model,
    MultipleOf,
    Name,
    NativeHint,
    NativeType,
    NodeType,
    NoteType,
    Notes,
    Nulls,
    OperatingSystem,
    Operations,
    Order,
    Otherwise,
    Output,
    Outputs,
    PageEnd,
    PageStart,
    Pagination,
    Parameters,
    ParentItem,
    ParentOrganization,
    Parts,
    Path,
    Pattern,
    Position,
    PostOfficeBoxNumber,
    PostalCode,
    ProductId,
    ProgrammingLanguage,
    PropertyId,
    Provenance,
    ProvenanceCategory,
    Publisher,
    QualityWeight,
    RandomSeed,
    References,
    Rel,
    Replacement,
    Replicates,
    Returns,
    ReviewAspect,
    Reviews,
    Role,
    RoleName,
    RowSpan,
    RowType,
    Rows,
    RuntimePlatform,
    SectionType,
    Select,
    SemanticDigest,
    SoftwareRequirements,
    SoftwareVersion,
    Source,
    SpeedWeight,
    Sponsors,
    StackTrace,
    StartColumn,
    StartLine,
    StartPosition,
    StateDigest,
    StreetAddress,
    StyleLanguage,
    SuggestionStatus,
    Suggestions,
    Target,
    TargetProducts,
    TelephoneNumbers,
    Temperature,
    TermCode,
    Text,
    Thumbnail,
    TimeUnit,
    TimeUnits,
    Title,
    Transcript,
    Type,
    UniqueItems,
    Url,
    Validator,
    Value,
    Values,
    Variable,
    Version,
    VolumeNumber,
}
