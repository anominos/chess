import Piece from "./Piece";

type SquareProps = {
  index: number;
  showPiece: (index: number) => boolean;
};

export default function Square({ index, showPiece }: SquareProps) {
  const isDark = (Math.floor(index / 8) + index) % 2 === 1;

  return (
    <div
      className={`flex h-full w-full items-center justify-center ${
        isDark
          ? "bg-slate-700 text-slate-100 dark:bg-slate-700 dark:text-slate-100"
          : "bg-slate-100 text-slate-900 dark:bg-slate-800 dark:text-slate-100"
      }`}
    >
      <span className="text-xs font-medium opacity-60">
        {showPiece(index) && <Piece />}
      </span>
    </div>
  );
}
