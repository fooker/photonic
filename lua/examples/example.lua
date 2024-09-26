local example = {}

function example:init()

end

function example:update(ctx, out)
    size = out.size

    -- out:update(function (i, _)
    --    return i/size, 0, (size-i)/size
    -- end)

    for i = 0, out.size do
        out[i] = { i/size, 0, (size-i)/size }
    end

    for i = 0, out.size, 7 do
        out[i] = { 0, 1, 0 }
    end
end

return example
