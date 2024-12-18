import React, { useState } from "react";
import { useNavigate } from "react-router-dom";
import axios from "axios";

export default function ActionsTab({ValueText, search, setSearch, selected, setSelected, PathRedirect, UrlApi}) {
    const [open, setOpen] = useState(false);

    const navigate = useNavigate();

    const handleClose = () => {
        setOpen(!open);
    };

    const handleRemove = async () => {
        try {
            await Promise.all(
                selected.map(async (valueId) => {
                    await axios.delete(UrlApi, {
                        headers: {
                            accept: "*/*",
                            "Content-Type": "application/json",
                        },
                        data: {
                            id: valueId,
                        },
                    });
                })
            );
            setSelected([]);
            window.location.reload();
        } catch (error) {
            console.error("Error removing " + ValueText, error);
        }
    }

    const handleRedirect = () => {
        navigate(PathRedirect);
    }

    return (
        <div className="flex flex-row gap-4 my-2 mx-4">
            <input
              type="text"
              id="search"
              name="search"
              placeholder={`Search ${ValueText}`}
              className="input input-bordered w-full max-w-xs flex-grow"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
            />
            <div className="ml-auto flex gap-2">
              <button
                className="btn btn-success"
                onClick={handleRedirect}
              >
                Add {ValueText}
              </button>

              <button
                className="btn btn-error"
                onClick={() =>
                  document.getElementById("remove_coach_modal").showModal()
                }
                disabled={selected.length === 0}
              >
                Remove {ValueText}
              </button>
              <dialog id="remove_coach_modal" className="modal justify-center text-center">
                <div className="modal-box">
                  <h3 className="font-bold text-lg">Confirmer la suppression:</h3>
                  <p className="py-4">
                    Are you sûre you want to remove {ValueText} ?
                  </p>
                    <form method="dialog">
                      <div className="flex justify-center gap-10">
                        <button
                          className="btn btn-error"
                          onClick={handleRemove}
                        >
                          Remove
                        </button>
                        <button className="btn btn-warning" onClick={handleClose}>
                          Cancel
                        </button>
                      </div>
                    </form>
                </div>
              </dialog>
            </div>
        </div>
    );
}