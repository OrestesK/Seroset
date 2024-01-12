export const metadata = {
    title: "Authenticate",
    description: "Auth",
};

export default function RootLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    return (
        <div>
            <h1 className="text-center border-black border-2 mt-2 text-2xl w-1/2 ml-auto mr-auto">
                Auth
            </h1>
            {children}
        </div>
    );
}
