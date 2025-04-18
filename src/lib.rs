use zed_extension_api as zed;

const PRISMA_PACKAGE: &str = "prisma";
const CLI_PATH: &str = "node_modules/prisma/build/index.js";

struct PrismaModelContextExtension;

impl zed::Extension for PrismaModelContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &zed::ContextServerId,
        _project: &zed::Project,
    ) -> zed::Result<zed::Command> {
        let latest_version = zed::npm_package_latest_version(PRISMA_PACKAGE)?;
        let installed_version = zed::npm_package_installed_version(PRISMA_PACKAGE)?;

        if installed_version.as_ref() != Some(&latest_version) {
            zed::npm_install_package(PRISMA_PACKAGE, &latest_version)?;
        }

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![
                std::env::current_dir()
                    .map_err(|err| err.to_string())?
                    .join(CLI_PATH)
                    .to_str()
                    .ok_or("Prisma CLI path must be valid UTF-8")?
                    .into(),
                "mcp".into(),
            ],
            env: std::env::vars().collect(),
        })
    }
}

zed::register_extension!(PrismaModelContextExtension);
