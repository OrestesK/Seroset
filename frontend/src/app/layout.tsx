import "./globals.css";

export const metadata = {
  title: "Seroset",
  description: "TODO",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en-US">
      <body>
        <h1 className="text-center border-black border-2 mt-5 text-4xl">
          Seroset
        </h1>
        {children}
      </body>
    </html>
  );
}
