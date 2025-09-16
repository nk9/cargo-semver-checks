pub enum EnumWithMustUseMethods {
    Bar,
}

impl EnumWithMustUseMethods {
    // These methods did not have the #[must_use] attribute in the old version.
    // Addition of the attribute should NOT be reported.
    pub fn AssociatedFnToMustUse() {}

    pub fn AssociatedFnToMustUseMessage() {}

    pub fn MethodToMustUseMethod(&self) {}

    pub fn MethodToMustUseMessageMethod(&self) {}

    // This method had the #[must_use] attribute in the old version. Changes
    // of the attribute should not be reported.

    #[must_use]
    pub fn MustUseMethodToMustUseMessageMethod(&self) {}

    // These methods had the #[must_use] attribute in the old version.
    // They also included the user-defined warning message. Changes of
    // the attribute, including deletion, should not be reported.

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMustUseMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMustUseMessageMethod(&self) {}


    // These methods had the #[must_use] attribute in the old version.
    // Deletion of the attribute SHOULD be reported.
    #[must_use]
    pub fn MustUseMethodToMethod(&self) {}

    #[must_use]
    pub fn MustUseMessageMethodToMethod(&self) {}
}

// This public enum's inherent method had the #[must_use] attribute
// in the old version. Because the method is private, removing the attribute
// should NOT be reported.

pub enum EnumWithPrivateMustUseMethods {
    Bar,
}

impl EnumWithPrivateMustUseMethods {
    fn PrivateMustUseMethodToPrivateMethod(&self) {}
}

// This enum is private and adding #[must_use] to its inherent method
// should NOT be reported.

enum PrivateEnumWithMustUseMethods {
    Bar,
}

impl PrivateEnumWithMustUseMethods {
    fn PrivateEnumMethodToPrivateEnumMustUseMethod(&self) {}
}

// This enum and its inherent method were removed from the new version of the
// crate, together with the method's attribute.
// It should NOT be reported by this rule to avoid duplicate lints.

pub enum RemovedEnumWithMustUseMethods {
    Bar,
}

impl RemovedEnumWithMustUseMethods {
    #[must_use]
    pub fn RemovedEnumMustUseAssociatedFn() {}

    #[must_use]
    pub fn RemovedEnumMustUseMethod(&self) {}
}
