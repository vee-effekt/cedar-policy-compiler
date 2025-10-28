// Example Cedar policy: Admins can do anything
permit(
    principal,
    action,
    resource
)
when {
    principal.role == "admin"
};
