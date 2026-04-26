function zeroPad(num: number) {
    return num.toString().padStart(2, "0");
}

function DateDisplay({ date }: { date: Date }) {
    return (
        <>
            {date.getFullYear()}-{date.getMonth()}-{date.getDate()}{" "}
            {zeroPad(date.getHours())}:{zeroPad(date.getMinutes())}
        </>
    );
}

export { DateDisplay };
