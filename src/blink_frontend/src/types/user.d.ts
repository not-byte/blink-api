import Language from "@/types/language";
import Status from "@/types/status";
import Theme from "@/types/theme";
import type { Principal } from "@dfinity/principal";

interface User {
    principal: Principal;
    username: string;
    avatar: string;
    language: Language;
    theme: Theme;
    status: Status;
}
