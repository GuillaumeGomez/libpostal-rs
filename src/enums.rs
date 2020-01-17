use sys;
use traits::ToC;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AddressComponent {
    Any,
    Name,
    HouseNumber,
    Street,
    Unit,
    Level,
    Staircase,
    Entrance,
    Category,
    Near,
    Toponym,
    PostalCode,
    POBox,
    All,
}

impl ToC for AddressComponent {
    type Out = u16;

    fn to_c(&self) -> u16 {
        match *self {
            AddressComponent::Any => sys::LIBPOSTAL_ADDRESS_ANY,
            AddressComponent::Name => sys::LIBPOSTAL_ADDRESS_NAME,
            AddressComponent::HouseNumber => sys::LIBPOSTAL_ADDRESS_HOUSE_NUMBER,
            AddressComponent::Street => sys::LIBPOSTAL_ADDRESS_STREET,
            AddressComponent::Unit => sys::LIBPOSTAL_ADDRESS_UNIT,
            AddressComponent::Level => sys::LIBPOSTAL_ADDRESS_LEVEL,
            AddressComponent::Staircase => sys::LIBPOSTAL_ADDRESS_STAIRCASE,
            AddressComponent::Entrance => sys::LIBPOSTAL_ADDRESS_ENTRANCE,
            AddressComponent::Category => sys::LIBPOSTAL_ADDRESS_CATEGORY,
            AddressComponent::Near => sys::LIBPOSTAL_ADDRESS_NEAR,
            AddressComponent::Toponym => sys::LIBPOSTAL_ADDRESS_TOPONYM,
            AddressComponent::PostalCode => sys::LIBPOSTAL_ADDRESS_POSTAL_CODE,
            AddressComponent::POBox => sys::LIBPOSTAL_ADDRESS_PO_BOX,
            AddressComponent::All => sys::LIBPOSTAL_ADDRESS_ALL,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DuplicateStatus {
    Null,
    NonDuplicate,
    PossibleDuplicateNeedsReview,
    LikelyDuplicate,
    ExactDuplicate,
}

impl ToC for DuplicateStatus {
    type Out = sys::libpostal_duplicate_status_t;

    fn to_c(&self) -> sys::libpostal_duplicate_status_t {
        match *self {
            DuplicateStatus::Null => {
                sys::libpostal_duplicate_status_t::LIBPOSTAL_NULL_DUPLICATE_STATUS
            }
            DuplicateStatus::NonDuplicate => {
                sys::libpostal_duplicate_status_t::LIBPOSTAL_NON_DUPLICATE
            }
            DuplicateStatus::PossibleDuplicateNeedsReview => {
                sys::libpostal_duplicate_status_t::LIBPOSTAL_POSSIBLE_DUPLICATE_NEEDS_REVIEW
            }
            DuplicateStatus::LikelyDuplicate => {
                sys::libpostal_duplicate_status_t::LIBPOSTAL_LIKELY_DUPLICATE
            }
            DuplicateStatus::ExactDuplicate => {
                sys::libpostal_duplicate_status_t::LIBPOSTAL_EXACT_DUPLICATE
            }
        }
    }
}
